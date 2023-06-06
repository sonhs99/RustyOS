use core::arch::asm;
use core::{mem::size_of, ptr::write_bytes};
use x86_64::registers::segmentation::Segment;
use x86_64::structures::gdt::{Descriptor, SegmentSelector};

use crate::println;

#[repr(C, packed(8))]
pub struct Context {
    gs: u64,
    fs: u64,
    es: u64,
    ds: u64,
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rsi: u64,
    rdi: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,
    rbp: u64,
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

impl Context {
    pub fn new() -> Self {
        Context {
            gs: 0,
            fs: 0,
            es: 0,
            ds: 0,
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            r11: 0,
            r10: 0,
            r9: 0,
            r8: 0,
            rsi: 0,
            rdi: 0,
            rdx: 0,
            rcx: 0,
            rbx: 0,
            rax: 0,
            rbp: 0,
            rip: 0,
            cs: 0,
            rflags: 0,
            rsp: 0,
            ss: 0,
        }
    }
    pub fn save(&mut self) {
        use x86_64::registers::{read_rip, rflags, segmentation};
        unsafe { asm!("push rax") }
        self.ss = u64::from(segmentation::SS::get_reg().0);
        unsafe {
            asm!(
                "mov {}, rbp",
                out(reg) self.rsp
            );
        }
        self.rflags = rflags::read_raw();
        self.cs = u64::from(segmentation::CS::get_reg().0);
        self.rip = read_rip().as_u64();
        unsafe { asm!("pop rax", "pop rbp") }

        unsafe {
            asm!("add rdi, (19 * 8)", "mov rsp, rdi", "pop rdi");
            println!("Create Task Start");
            asm!(
                "push rbp", "push rax", "push rbx", "push rcx", "push rdx", "push rdi", "push rsi",
                "push r8", "push r9", "push r10", "push r11", "push r12", "push r13", "push r14",
                "push r15",
            );
            asm!(
                "mov ax, ds",
                "push rax",
                "mov ax, es",
                "push rax",
                "push fs",
                "push gs",
            );
        }
    }
    pub unsafe fn load(&self) {
        use x86_64::registers::segmentation;
        segmentation::GS::set_reg(SegmentSelector(self.gs as u16));
        segmentation::FS::set_reg(SegmentSelector(self.fs as u16));
        segmentation::ES::set_reg(SegmentSelector(self.es as u16));
        segmentation::DS::set_reg(SegmentSelector(self.ds as u16));

        asm!(
            "mov rsp, {}",
            in(reg) self as *const Self
        );
        asm!(
            "pop r15", "pop r14", "pop r13", "pop r12", "pop r11", "pop r10", "pop r9", "pop r8",
            "pop rsi", "pop rdi", "pop rdx", "pop rcx", "pop rbx", "pop rax", "pop rbp",
        );
        asm!("iretq");
    }
}

pub struct TCB {
    pub context: Context,
    id: u64,
    flags: u64,
    stack_address: u64,
    stack_size: u64,
}

impl TCB {
    pub fn new() -> Self {
        TCB {
            context: Context::new(),
            id: 0,
            flags: 0,
            stack_address: 0,
            stack_size: 0,
        }
    }
    pub fn init(
        &mut self,
        id: u64,
        flags: u64,
        entry_point: u64,
        stack_address: u64,
        stack_size: u64,
    ) {
        self.context = Context::new();
        self.context.rsp = stack_address + stack_size;
        self.context.rbp = stack_address + stack_size;

        self.context.cs = match Descriptor::kernel_code_segment() {
            Descriptor::UserSegment(seg) => seg,
            _ => 0,
        };
        self.context.ds = match Descriptor::kernel_data_segment() {
            Descriptor::UserSegment(seg) => seg,
            _ => 0,
        };
        self.context.es = match Descriptor::kernel_data_segment() {
            Descriptor::UserSegment(seg) => seg,
            _ => 0,
        };
        self.context.fs = match Descriptor::kernel_data_segment() {
            Descriptor::UserSegment(seg) => seg,
            _ => 0,
        };
        self.context.gs = match Descriptor::kernel_data_segment() {
            Descriptor::UserSegment(seg) => seg,
            _ => 0,
        };
        self.context.ss = match Descriptor::kernel_data_segment() {
            Descriptor::UserSegment(seg) => seg,
            _ => 0,
        };

        self.context.rip = entry_point;
        self.context.rflags |= 0x0200;

        self.id = id;
        self.stack_address = stack_address;
        self.stack_size = stack_size;
        self.flags = flags;
    }
}
