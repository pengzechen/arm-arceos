use crate::{irq::IrqHandler, mem::phys_to_virt};
use arm_gic::gic_v2::{GicCpuInterface, GicDistributor, GicHypervisorInterface};
use memory_addr::PhysAddr;
use spinlock::SpinNoIrq;

// use hypercraft::arch;  // shade ajax-2024/3/7

/// The maximum number of IRQs.
pub const MAX_IRQ_COUNT: usize = 1024;

/// The timer IRQ number.
pub const TIMER_IRQ_NUM: usize = 30; // physical timer, type=PPI, id=14

/// The hypervisor timer irq number.
pub const HYPERVISOR_TIMER_IRQ_NUM: usize = 26;

/// The ipi irq number.
pub const IPI_IRQ_NUM: usize = 1;

/// The maintenance interrupt irq number.
pub const MAINTENANCE_IRQ_NUM: usize = 25;

const GICD_BASE: PhysAddr = PhysAddr::from(axconfig::GICD_PADDR);
const GICC_BASE: PhysAddr = PhysAddr::from(axconfig::GICC_PADDR);

// const GICH_BASE: PhysAddr = PhysAddr::from(axconfig::GICH_PADDR);  // shade ajax-2024/3/7

pub static GICD: 
SpinNoIrq<GicDistributor> = SpinNoIrq::new(GicDistributor::new(phys_to_virt(GICD_BASE).as_mut_ptr()));

// per-CPU, no lock
pub static GICC: 
GicCpuInterface = GicCpuInterface::new(phys_to_virt(GICC_BASE).as_mut_ptr());

// pub static GICH: GicHypervisorInterface = GicHypervisorInterface::new(phys_to_virt(GICH_BASE).as_mut_ptr()); // shade ajax-2024/3/7

/// Enables or disables the given IRQ.
pub fn set_enable(irq_num: usize, enabled: bool) {
    GICD.lock().set_enable(irq_num as _, enabled);

    /* shade ajax-2024/3/7 */
    // GICD.lock().set_priority(irq_num as _, 0x7f);
    // GICD.lock().set_target_cpu(irq_num as _, 1 << 0);   // only enable one cpu
    // GICD.lock().set_enable(irq_num as _, enabled);
}

/// Registers an IRQ handler for the given IRQ.
///
/// It also enables the IRQ if the registration succeeds. It returns `false` if
/// the registration failed.
pub fn register_handler(irq_num: usize, handler: IrqHandler) -> bool {
    crate::irq::register_handler_common(irq_num, handler)
}

/// Dispatches the IRQ.
///
/// This function is called by the common interrupt handler. It looks
/// up in the IRQ handler table and calls the corresponding handler. If
/// necessary, it also acknowledges the interrupt controller after handling.
pub fn dispatch_irq(_unused: usize) {
    GICC.handle_irq(|irq_num| crate::irq::dispatch_irq_common(irq_num as _));
}

/// Initializes GICD, GICC on the primary CPU.
pub(crate) fn init_primary() {
    info!("Initialize GICv2...");
    GICD.lock().init();
    GICC.init();

    /* shade ajax-2024/3/7 */
    // GICH.init();
    // arch::GICH = Some(&GICH);
    // arch::GICC = Some(&GICC);
    // arch::GICD = Some(&GICD);
}

pub(crate) fn init_secondary() {
    GICC.init();
}
