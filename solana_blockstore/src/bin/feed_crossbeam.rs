use {
    crossbeam_channel::{Receiver, Sender},
    std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        thread::{self, Builder, JoinHandle},
        time::Duration,
    },
};

/// Timeout to wait on the poh timing points from the channel
const POH_TIMING_RECEIVER_TIMEOUT_MILLISECONDS: u64 = 1000;

#[derive(Clone, Debug)]
pub struct SlotPohTimingInfo {
    /// current slot
    pub slot: u64,
    /// timing event
    pub timing_point: u64,
}

impl SlotPohTimingInfo {
    pub fn new_slot_start_poh_time_point(slot: u64, timing_point: u64) -> SlotPohTimingInfo {
        SlotPohTimingInfo { slot, timing_point }
    }
}

/// Receiver of SlotPohTimingInfo from the channel
pub type PohTimingReceiver = Receiver<SlotPohTimingInfo>;

/// Sender of SlotPohTimingInfo to the channel
pub type PohTimingSender = Sender<SlotPohTimingInfo>;

/// send poh timing to channel
pub fn send_poh_timing_point(sender: &PohTimingSender, slot_timing: SlotPohTimingInfo) {
    if let Err(e) = sender.try_send(slot_timing) {
        dbg!("failed to send slot poh timing {:?}", e);
    }
}

pub struct PohTimingReportService {
    t_poh_timing: JoinHandle<()>,
}

impl PohTimingReportService {
    pub fn new(receiver: PohTimingReceiver, exit: &Arc<AtomicBool>) -> Self {
        let exit_signal = exit.clone();
        // let mut poh_timing_reporter = PohTimingReporter::default();
        let t_poh_timing = Builder::new()
            .name("solPohTimingRpt".to_string())
            .spawn(move || loop {
                if exit_signal.load(Ordering::Relaxed) {
                    break;
                }
                if let Ok(SlotPohTimingInfo {
                    slot: 10,
                    timing_point: 1024,
                }) = receiver.recv_timeout(Duration::from_millis(
                    POH_TIMING_RECEIVER_TIMEOUT_MILLISECONDS,
                )) {
                    // poh_timing_reporter.process(slot, root_slot, timing_point);
                }
            })
            .unwrap();
        Self { t_poh_timing }
    }
    pub fn join(self) -> thread::Result<()> {
        self.t_poh_timing.join()
    }
}

fn main() {}

#[test]
fn test_poh_timing_report_service() {
    let (poh_timing_point_sender, poh_timing_point_receiver) = unbounded();
    let exit = Arc::new(AtomicBool::new(false));
    // Create the service
    let poh_timing_report_service = PohTimingReportService::new(poh_timing_point_receiver, &exit);

    // Send SlotPohTimingPoint
    let _ = poh_timing_point_sender.send(SlotPohTimingInfo::new_slot_start_poh_time_point(42, 100));
    let _ = poh_timing_point_sender.send(SlotPohTimingInfo::new_slot_start_poh_time_point(42, 200));
    let _ = poh_timing_point_sender.send(SlotPohTimingInfo::new_slot_start_poh_time_point(42, 150));

    // Shutdown the service
    exit.store(true, Ordering::Relaxed);
    poh_timing_report_service
        .join()
        .expect("poh_timing_report_service completed");
}
