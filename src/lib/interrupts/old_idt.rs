//! This is a hand-done idt that didn't work because I was unable to get the stack alignment
use bit_field::BitField;
use core::mem::size_of;
use lazy_static::lazy_static;
use x86_64::instructions::segmentation;
use x86_64::instructions::tables::{lidt, DescriptorTablePointer};
use x86_64::structures::gdt::SegmentSelector;
use x86_64::PrivilegeLevel;

pub struct Idt([Entry; 16]); //* 16 entry array

impl Idt {
    pub fn new() -> Idt {
        Idt([Entry::missing(); 16])
    }
}

impl Idt {
    pub fn load(&self) {
        let ptr = DescriptorTablePointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };

        unsafe { lidt(&ptr) }; // load idt
    }

    pub fn set_handler(&mut self, entry: u8, handler: HandlerFunc) -> &mut EntryOptions {
        unsafe {
            self.0[entry as usize] = Entry::new(segmentation::cs(), handler);
            &mut self.0[entry as usize].options
        }
    }
}

pub type HandlerFunc = extern "C" fn() -> !;

#[derive(Debug, Clone, Copy)] //* allows for these traits
#[repr(C, packed)] //* no padding
pub struct Entry {
    low_pointer: u16,              //* lower section of offset
    gdt_selector: SegmentSelector, //* code segment selector
    options: EntryOptions,         //* handleropts
    mid_pointer: u16,
    top_pointer: u32,
    reserved: u32, //* reserved, set to 0
}

impl Entry {
    fn new(gdt_selector: SegmentSelector, handler: HandlerFunc) -> Self {
        let pointer = handler as u64;
        Entry {
            gdt_selector: gdt_selector,
            low_pointer: pointer as u16,
            mid_pointer: (pointer >> 16) as u16,
            top_pointer: (pointer >> 32) as u32,
            options: EntryOptions::new(),
            reserved: 0,
        }
    }

    fn missing() -> Self {
        Entry {
            gdt_selector: SegmentSelector::new(0, PrivilegeLevel::Ring0),
            low_pointer: 0,
            mid_pointer: 0,
            top_pointer: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EntryOptions(u16);

impl EntryOptions {
    fn minimal() -> Self {
        let mut options = 0;
        options.set_bits(9..12, 0b111); // 'must-be-one' bits
        return EntryOptions(options);
    }

    fn new() -> Self {
        return *Self::minimal().set_present(true).disable_interrupts(true);
    }

    pub fn set_present(&mut self, present: bool) -> &mut Self {
        self.0.set_bit(15, present);
        self
    }

    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        self.0.set_bit(8, !disable);
        self
    }

    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0.set_bits(13..15, dpl);
        self
    }

    pub fn set_stack_index(&mut self, index: u16) -> &mut Self {
        self.0.set_bits(0..3, index);
        self
    }
}

pub fn init() {
    IDT.load();
}

#[derive(Debug)]
#[repr(C)]
struct ExceptionStackFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() -> ! {
            unsafe {
                llvm_asm!("mov rdi, rsp
                          sub rsp, 8 
                          call $0"::
                          "i"($name as extern "C" fn(
                            &ExceptionStackFrame) -> !):
                          "rdi" :
                          "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

extern "C" fn divide_by_zero_handler(stack_frame: &ExceptionStackFrame) -> ! {
    println!("EXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}

extern "C" fn breakpoint_handler() {
    println!("EXCEPTION: BREAKPOINT");
}

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        idt.set_handler(0, handler!(divide_by_zero_handler));

        idt
    };
}