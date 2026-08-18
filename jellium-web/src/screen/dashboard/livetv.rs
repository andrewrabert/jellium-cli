//! The Live TV administration: tuners, listing providers, channel mapping and
//! the DVR settings.

use iced::Element;
use iced::widget::{button, pick_list, row, text_input};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};
use jellium_model::appearance::typeface::Rank;
use jellium_model::form::{Field, Form};

use super::{Control, Group, Heading};

/// The Live TV administration, whichever tab is shown.
#[derive(Debug, Clone)]
pub struct State {
    pub tab: super::LiveTvTab,
    /// The tuner types the server offers, and the tuners it holds.
    pub types: Vec<jellyfin_api::types::NameIdPair>,
    pub tuners: Vec<jellyfin_api::types::TunerHostInfo>,
    /// The tuners a discovery answered with.
    pub discovered: Vec<jellyfin_api::types::TunerHostInfo>,
    /// The listing providers held, and what a new one is being given.
    pub providers: Vec<jellyfin_api::types::ListingsProviderInfo>,
    pub provider: Provider,
    /// The lineups the server reported for the provider being added.
    pub lineups: Vec<jellyfin_api::types::NameIdPair>,
    /// The countries Schedules Direct offers.
    pub countries: Vec<String>,
    /// What the mapping tab offers, and the mappings held.
    pub mapping: Option<jellyfin_api::types::ChannelMappingOptionsDto>,
    /// The DVR settings, read whole and written whole.
    pub dvr: Form,
    /// What a tuner is being given.
    pub tuner_url: String,
    pub tuner_type: String,
}

/// What a listing provider is being added as.
#[derive(Debug, Clone, Default)]
pub struct Provider {
    /// True for Schedules Direct, false for XMLTV.
    pub schedules_direct: bool,
    pub username: String,
    pub password: String,
    pub country: String,
    pub postcode: String,
    pub lineup: String,
    /// The XMLTV path.
    pub path: String,
}

/// The DVR settings the reference draws, in its order; every key outside them
/// survives a save.
// reference: dvr-recording-paths
// reference: dvr-recording-padding
pub const DVR: &[Group] = &[
    Group {
        heading: None,
        note: None,
        controls: &[
            Control {
                field: Field::Text {
                    key: "RecordingPath",
                },
                label: Text::DvrRecordingPath,
                helper: &[Text::DvrRecordingPathHelp],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Text {
                    key: "MovieRecordingPath",
                },
                label: Text::DvrMovieRecordingPath,
                helper: &[],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Text {
                    key: "SeriesRecordingPath",
                },
                label: Text::DvrSeriesRecordingPath,
                helper: &[],
                unit: None,
                offered: None,
            },
        ],
        closing: None,
    },
    Group {
        heading: Some(Heading {
            rank: Rank::Second,
            title: Text::DvrDefaults,
        }),
        note: None,
        controls: &[
            Control {
                field: Field::Minutes {
                    key: "PrePaddingSeconds",
                },
                label: Text::DvrStartWhenPossible,
                helper: &[],
                unit: Some(Text::DvrMinutesBefore),
                offered: None,
            },
            Control {
                field: Field::Minutes {
                    key: "PostPaddingSeconds",
                },
                label: Text::DvrStopWhenPossible,
                helper: &[],
                unit: Some(Text::DvrMinutesAfter),
                offered: None,
            },
        ],
        closing: None,
    },
];

pub async fn load(api: std::rc::Rc<crate::api::Api>, tab: super::LiveTvTab) -> Answer<State> {
    Answer::of(async {
        let countries = match tab {
            super::LiveTvTab::Providers => api
                .schedules_direct_countries()
                .await
                .map(|held| {
                    held.as_object()
                        .map(|held| held.keys().cloned().collect())
                        .unwrap_or_default()
                })
                .or_default(Text::FailureProvidersUnread),
            _ => Vec::new(),
        };
        let mapping = match tab {
            super::LiveTvTab::Mapping => {
                let provider = api
                    .providers()
                    .await
                    .or_default(Text::FailureProvidersUnread)
                    .into_iter()
                    .find_map(|provider| provider.id);
                match provider {
                    Some(provider) => api
                        .mapping_options(&provider)
                        .await
                        .or_none(Text::FailureMappingUnread),
                    None => None,
                }
            }
            _ => None,
        };
        Ok(State {
            tab,
            types: api
                .tuner_types()
                .await
                .or_default(Text::FailureTunerTypesUnread),
            tuners: api.tuners().await.or_default(Text::FailureTunersUnread),
            discovered: Vec::new(),
            providers: api
                .providers()
                .await
                .or_default(Text::FailureProvidersUnread),
            provider: Provider::default(),
            lineups: Vec::new(),
            countries,
            mapping,
            dvr: Form::of(api.section("livetv").await.bubbled()?),
            tuner_url: String::new(),
            tuner_type: String::new(),
        })
    })
    .await
}

/// Whichever of the four screens is shown.
pub fn view<'a>(
    state: &'a State,
    read_only: bool,
    viewport: crate::style::Viewport,
) -> Vec<Element<'a, Message>> {
    let page: Page<'a> = vec![crate::widget::localnav(
        super::LiveTvTab::ALL
            .into_iter()
            .map(|tab| crate::widget::Entry {
                label: tab.label(),
                showing: match tab == state.tab {
                    true => crate::widget::Showing::Shown,
                    false => crate::widget::Showing::Offered(Message::DashboardAction(
                        super::Action::Open(super::Screen::LiveTv { tab }),
                    )),
                },
            }),
    )];
    match state.tab {
        super::LiveTvTab::Tuners => tuners(page, state, read_only),
        super::LiveTvTab::Providers => providers(page, state, read_only),
        super::LiveTvTab::Mapping => mapping(page, state, read_only),
        super::LiveTvTab::Dvr => dvr(page, state, read_only, viewport),
    }
}

type Page<'a> = Vec<Element<'a, Message>>;

fn tuners<'a>(mut page: Page<'a>, state: &'a State, read_only: bool) -> Page<'a> {
    for tuner in &state.tuners {
        let Some(id) = tuner.id.clone() else {
            continue;
        };
        let url = tuner.url.clone().unwrap_or_default();
        let mut held = row![
            prose(url.clone(), typeface::BODY),
            prose(tuner.type_.clone().unwrap_or_default(), typeface::BODY),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()));
        if !read_only {
            held = held.push(
                button(prose(strings::lookup(Text::TunersReset), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::Write(
                        super::Written::ResetTuner {
                            id: id.clone(),
                            name: url.clone(),
                        },
                    ))),
            );
            held = held.push(
                button(prose(strings::lookup(Text::TunersDelete), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::Ask(
                        crate::screen::confirm::Pending::of(
                            crate::screen::confirm::Destructive::DeleteTuner { id },
                            url,
                        ),
                    ))),
            );
        }
        page.push(held.into());
    }

    if read_only {
        return page;
    }

    page.push(
        button(prose(strings::lookup(Text::TunersDiscover), typeface::BODY))
            .style(style::raised)
            .on_press(Message::DashboardAction(super::Action::Write(
                super::Written::DiscoverTuners,
            )))
            .into(),
    );
    for found in &state.discovered {
        page.push(prose(found.url.clone().unwrap_or_default(), typeface::BODY));
    }

    let types = state
        .types
        .iter()
        .filter_map(|kind| kind.id.clone())
        .collect::<Vec<_>>();
    page.push(
        row![
            pick_list(types, Some(state.tuner_type.clone()), |chosen| {
                Message::DashboardAction(super::Action::TypedPassword(chosen))
            }),
            text_input(strings::lookup(Text::TunersUrl), &state.tuner_url)
                .style(style::input)
                .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
            button(prose(strings::lookup(Text::TunersAdd), typeface::BODY))
                .style(style::submit)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::AddTuner {
                        url: state.tuner_url.clone(),
                        kind: state.tuner_type.clone(),
                    }
                ))),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()))
        .into(),
    );
    page
}

fn providers<'a>(mut page: Page<'a>, state: &'a State, read_only: bool) -> Page<'a> {
    for provider in &state.providers {
        let Some(id) = provider.id.clone() else {
            continue;
        };
        let named = provider.type_.clone().unwrap_or_default();
        let mut held = row![prose(named.clone(), typeface::BODY)]
            .spacing(style::drawn(space::CONTROL_GAP.drawn()));
        if !read_only {
            held = held.push(
                button(prose(
                    strings::lookup(Text::ProvidersDelete),
                    typeface::BODY,
                ))
                .style(style::raised)
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::DeleteProvider { id },
                        named,
                    ),
                ))),
            );
        }
        page.push(held.into());
    }

    if read_only {
        return page;
    }

    page.push(
        row![
            button(prose(
                strings::lookup(Text::ProvidersSchedulesDirect),
                typeface::BODY
            ))
            .style(style::flat)
            .on_press(Message::DashboardAction(super::Action::ProviderKind(true))),
            button(prose(strings::lookup(Text::ProvidersXmltv), typeface::BODY))
                .style(style::flat)
                .on_press(Message::DashboardAction(super::Action::ProviderKind(false))),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()))
        .into(),
    );

    if state.provider.schedules_direct {
        page.push(
            text_input(
                strings::lookup(Text::ProvidersUsername),
                &state.provider.username,
            )
            .style(style::input)
            .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed)))
            .into(),
        );
        page.push(
            text_input(
                strings::lookup(Text::ProvidersPassword),
                &state.provider.password,
            )
            .style(style::input)
            .secure(true)
            .on_input(|typed| Message::DashboardAction(super::Action::TypedPassword(typed)))
            .into(),
        );
        page.push(
            pick_list(
                state.countries.clone(),
                Some(state.provider.country.clone()),
                |chosen| Message::DashboardAction(super::Action::ProviderCountry(chosen)),
            )
            .into(),
        );
        page.push(
            text_input(
                strings::lookup(Text::ProvidersPostcode),
                &state.provider.postcode,
            )
            .style(style::input)
            .on_input(|typed| Message::DashboardAction(super::Action::ProviderPostcode(typed)))
            .into(),
        );
        page.push(
            button(prose(
                strings::lookup(Text::ProvidersLineup),
                typeface::BODY,
            ))
            .style(style::raised)
            .on_press(Message::DashboardAction(super::Action::Write(
                super::Written::FetchLineups,
            )))
            .into(),
        );
        for lineup in &state.lineups {
            page.push(widget::anchor(
                lineup.name.clone().unwrap_or_default(),
                Message::DashboardAction(super::Action::ProviderLineup(
                    lineup.id.clone().unwrap_or_default(),
                )),
            ));
        }
    } else {
        page.push(
            text_input(strings::lookup(Text::ProvidersPath), &state.provider.path)
                .style(style::input)
                .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed)))
                .into(),
        );
    }

    page.push(
        button(prose(strings::lookup(Text::ProvidersAdd), typeface::BODY))
            .style(style::submit)
            .on_press(Message::DashboardAction(super::Action::Write(
                super::Written::AddProvider,
            )))
            .into(),
    );
    page
}

fn mapping<'a>(mut page: Page<'a>, state: &'a State, read_only: bool) -> Page<'a> {
    let Some(options) = state.mapping.as_ref() else {
        return page;
    };

    let providers = options
        .provider_channels
        .iter()
        .filter_map(|channel| channel.name.clone())
        .collect::<Vec<_>>();

    for channel in &options.tuner_channels {
        let Some(id) = channel.id.clone() else {
            continue;
        };
        let mut held = row![prose(
            channel.name.clone().unwrap_or_default(),
            typeface::BODY
        )]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()));
        if !read_only {
            let tuner = id.clone();
            held = held.push(pick_list(
                providers.clone(),
                None::<String>,
                move |chosen| {
                    Message::DashboardAction(super::Action::Write(super::Written::MapChannel {
                        tuner: tuner.clone(),
                        provider: chosen,
                    }))
                },
            ));
        }
        page.push(held.into());
    }
    page
}

// the reference writes the unit inside the field as a trailing adornment;
// here it stands under the field as its own sentence
// reference: dvr-recording-padding
fn dvr<'a>(
    mut page: Page<'a>,
    state: &'a State,
    read_only: bool,
    viewport: crate::style::Viewport,
) -> Page<'a> {
    page.extend(super::controls(
        DVR,
        &state.dvr,
        super::Controls::Mui,
        viewport,
    ));
    if !read_only {
        page.push(super::save(
            super::Controls::Mui,
            Some(Message::DashboardAction(super::Action::Save)),
            viewport.layout(),
        ));
    }
    page
}
