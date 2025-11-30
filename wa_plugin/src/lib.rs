pub mod colors;
mod editor;
pub mod fonts;
pub mod styles;

use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use std::{
    fmt::Display,
    sync::{Arc, RwLock},
};
use walkanalysis::{
    exercise::{analysis::Analysis, arpeggios_up::ArpeggiosUp, Exercise},
    form::{
        form::Form,
        songs::{autumn_leaves::autumn_leaves, but_beautiful::but_beautiful, test::test},
    },
    transcribe::transcribe::{AudioSettings, Transcription, DEFAULT_SETTINGS},
};

use crate::editor::WalkanalysisSharedState;

#[derive(Debug, Enum, PartialEq, Clone, Copy, Eq)]
pub enum ExerciseKind {
    ArpeggiosUp,
}

unsafe impl Sync for ExerciseKind {}

impl ExerciseKind {
    pub fn exercise(&self) -> Box<dyn Exercise> {
        match self {
            ExerciseKind::ArpeggiosUp => Box::new(ArpeggiosUp {}),
        }
    }

    const ALL: [ExerciseKind; 1] = [ExerciseKind::ArpeggiosUp];
}

impl Display for ExerciseKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExerciseKind::ArpeggiosUp => write!(f, "Arpeggios Up"),
        }
    }
}

#[derive(Default, Debug, Enum, PartialEq, Clone, Copy, Eq)]
pub enum FormKind {
    #[default]
    Test,
    AutumnLeaves,
    AllTheThingsYouAre,
    ButBeautiful,
}

impl Display for FormKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormKind::Test => write!(f, "Test"),
            FormKind::AutumnLeaves => write!(f, "Autumn Leaves"),
            FormKind::AllTheThingsYouAre => write!(f, "All The Things You Are"),
            FormKind::ButBeautiful => write!(f, "But Beautiful"),
        }
    }
}

impl FormKind {
    pub fn form(&self) -> Form {
        match self {
            FormKind::AutumnLeaves => autumn_leaves(),
            FormKind::Test => test(),
            FormKind::AllTheThingsYouAre => todo!(),
            FormKind::ButBeautiful => but_beautiful(),
        }
    }

    const ALL: [FormKind; 3] = [
        FormKind::Test,
        // FormKind::AllTheThingsYouAre,
        FormKind::AutumnLeaves,
        FormKind::ButBeautiful,
    ];
}

unsafe impl Sync for FormKind {}

pub struct WalkAnalysis {
    params: Arc<WalkAnalysisParams>,
    /// While playing, gathers the data necessary to analyze the entire form.
    /// If _anything_ weird happens during playback (tempo changes, time goes backward),
    /// this is thrown out.
    /// Only starts if the user starts playing/recording at beat 0
    data: DataToAnalyze,
    form_cache: Option<FormCache>,
    state: Arc<RwLock<WalkanalysisSharedState>>,
}

impl WalkAnalysis {
    fn clear(&mut self) {
        self.data.clear();
        let mut state = self.state.write().unwrap();
        state.clear();
    }
}

pub struct FormCache {
    kind: FormKind,
    form: Form,
    length: u32,
}

pub enum DataAcquizitionState {
    WaitingForStart,
    Acquiring,
    Done,
}

pub struct DataToAnalyze {
    acquizition_state: DataAcquizitionState,
    tempo: Option<f64>,
    last_saved_beat_pos: Option<f64>,
    samples: Vec<f32>,
}

impl DataToAnalyze {
    pub fn clear(&mut self) {
        self.acquizition_state = DataAcquizitionState::WaitingForStart;
        self.tempo = None;
        self.last_saved_beat_pos = None;
        self.samples.clear();
    }
}

#[derive(Params)]
pub struct WalkAnalysisParams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
}

impl Default for WalkAnalysis {
    fn default() -> Self {
        Self {
            params: Arc::new(WalkAnalysisParams::default()),
            data: DataToAnalyze {
                acquizition_state: DataAcquizitionState::WaitingForStart,
                samples: Vec::with_capacity(48_000 * 60 * 3), // Allocate for 3 minutes of data, should be enough for most forms
                tempo: None,
                last_saved_beat_pos: None,
            },
            form_cache: None,
            state: Arc::new(RwLock::new(WalkanalysisSharedState {
                selected_form: FormKind::default(),
                selected_exercise: ExerciseKind::ArpeggiosUp,
                correction: None,
                beat_pos: None,
                analysis: None,
                form: FormKind::default().form(),
            })),
        }
    }
}

impl Default for WalkAnalysisParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for WalkAnalysis {
    const NAME: &'static str = "Walk Analysis";
    const VENDOR: &'static str = "Pieter Staal";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // The first audio IO layout is used as the default. The other layouts may be selected either
    // explicitly or automatically by the host or the user depending on the plugin API/backend.
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(1),
        main_output_channels: NonZeroU32::new(1),

        aux_input_ports: &[],
        aux_output_ports: &[],

        // Individual ports and the layout as a whole can be named here. By default these names
        // are generated as needed. This layout will be called 'Stereo', while a layout with
        // only one input and output channel would be called 'Mono'.
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.state.clone(), self.params.editor_state.clone())
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.
        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        if !(context.transport().playing) {
            return ProcessStatus::Normal;
        }

        if let DataAcquizitionState::WaitingForStart = self.data.acquizition_state {
            // determine whether to start data acquisition
            if context.transport().playing
                && context.transport().bar_number() == Some(0)
                && context.transport().pos_beats() <= Some(0.)
            {
                self.data.acquizition_state = DataAcquizitionState::Acquiring;
                let current_form = self.state.read().unwrap().selected_form;

                let form = current_form.form();
                let form_length = form.length_in_beats();
                self.form_cache = Some(FormCache {
                    kind: current_form,
                    form,
                    length: form_length,
                });
                println!(
                    "Started data acquisition for {}, {} measures",
                    current_form,
                    form_length / 4
                );
            }
        }

        if let DataAcquizitionState::Acquiring = self.data.acquizition_state {
            {
                let mut state = self.state.write().unwrap();
                state.beat_pos = context.transport().pos_beats();
            }

            // Take care of knowing the tempo of the data
            match (self.data.tempo, context.transport().tempo) {
                (None, None) => todo!(),
                (None, Some(transport_tempo)) => self.data.tempo = Some(transport_tempo),
                (Some(data_tempo), Some(transport_tempo)) => {
                    if data_tempo != transport_tempo {
                        println!(
                            "Tempo changed: was {}, is now {}, deleting data.",
                            data_tempo, transport_tempo
                        );
                        self.clear();
                        return ProcessStatus::Normal;
                    }
                }
                _ => (),
            }

            let Some(tempo) = self.data.tempo else {
                self.clear();
                println!("No tempo known at this point, cannot analyze.");
                return ProcessStatus::Normal;
            };

            // If time went backwards, throw out data
            if let Some(last_saved_beat_pos) = self.data.last_saved_beat_pos {
                let current_beat_pos = context.transport().pos_beats().unwrap_or(f64::MIN);
                if last_saved_beat_pos > current_beat_pos {
                    println!(
                        "Time went backwards, {} => {}",
                        last_saved_beat_pos, current_beat_pos
                    );
                    self.clear();
                    return ProcessStatus::Normal;
                }
            }

            // Save the incoming samples to the data to analyze
            for channel_samples in buffer.iter_samples() {
                for sample in channel_samples {
                    self.data.samples.push(*sample);
                    break; // Only read channel 0
                }
                self.data.last_saved_beat_pos = context.transport().pos_beats()
            }

            let Some(ref form_cache) = self.form_cache else {
                println!("No form cache found even though state is acquiring. Deleting data.");
                self.clear();
                return ProcessStatus::Normal;
            };

            let beats_per_second = tempo / 60.;
            let samples_per_beat = context.transport().sample_rate / beats_per_second as f32;
            let form_length_in_samples =
                (form_cache.length as f32 * samples_per_beat).ceil() as usize;

            if self.data.samples.len() >= form_length_in_samples {
                println!("Finished data acquisition for {:?}", form_cache.kind);
                self.data.acquizition_state = DataAcquizitionState::WaitingForStart;

                let (transcription, _) = Transcription::transcribe(
                    &self.data.samples,
                    tempo as f32,
                    DEFAULT_SETTINGS,
                    AudioSettings {
                        sample_rate: context.transport().sample_rate as u32,
                    },
                );

                let analysis = Analysis::analyze(transcription, &form_cache.form);
                let correction = self
                    .state
                    .read()
                    .unwrap()
                    .selected_exercise
                    .exercise()
                    .correct(&analysis);

                println!("{}", correction);
                // Analysis and correction of this run are ready, throw away old data.
                self.data.clear();

                {
                    let mut state = self.state.write().unwrap();
                    state.correction = Some(correction);
                    state.analysis = Some(analysis);
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for WalkAnalysis {
    const VST3_CLASS_ID: [u8; 16] = *b"WalkAnalysis1234";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Analyzer];
}

nih_export_vst3!(WalkAnalysis);
