//! The Live TV administration: tuners, listing providers, channel mapping and
//! the DVR settings.

use iced::widget::{button, column, pick_list, row, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;
use jellium_model::form::{Field, Form};

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

/// The fields the DVR settings expose; every key outside them survives a save.
pub const DVR: &[Field] = &[
    Field::Text {
        key: "RecordingPath",
    },
    Field::Text {
        key: "MovieRecordingPath",
    },
    Field::Text {
        key: "SeriesRecordingPath",
    },
    Field::Number {
        key: "PrePaddingSeconds",
    },
    Field::Number {
        key: "PostPaddingSeconds",
    },
    Field::Flag {
        key: "EnableRecordingSubfolders",
    },
    Field::Flag {
        key: "EnableOriginalAudioWithEncodedRecordings",
    },
    Field::Text {
        key: "RecordingEncodingFormat",
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

/// The four tabs, and whichever one is shown.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut tabs = row![].spacing(style::drawn(space::GUTTER.drawn()));
    for tab in super::LiveTvTab::ALL {
        let control = button(prose(strings::lookup(tab.label()), typeface::BODY));
        tabs = tabs.push(if tab == state.tab {
            control
        } else {
            control.on_press(Message::DashboardAction(super::Action::Open(
                super::Screen::LiveTv { tab },
            )))
        });
    }

    let mut page = column![tabs]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn()));

    page = match state.tab {
        super::LiveTvTab::Tuners => tuners(page, state, read_only),
        super::LiveTvTab::Providers => providers(page, state, read_only),
        super::LiveTvTab::Mapping => mapping(page, state, read_only),
        super::LiveTvTab::Dvr => dvr(page, state, read_only),
    };

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}

type Page<'a> = iced::widget::Column<'a, Message>;

fn tuners<'a>(mut page: Page<'a>, state: &'a State, read_only: bool) -> Page<'a> {
    page = page.push(prose(
        strings::lookup(Text::TunersTitle),
        typeface::HEADING_2,
    ));

    for tuner in &state.tuners {
        let Some(id) = tuner.id.clone() else {
            continue;
        };
        let url = tuner.url.clone().unwrap_or_default();
        let mut held = row![
            prose(url.clone(), typeface::BODY),
            prose(tuner.type_.clone().unwrap_or_default(), typeface::BODY),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()));
        if !read_only {
            held = held.push(
                button(prose(strings::lookup(Text::TunersReset), typeface::BODY)).on_press(
                    Message::DashboardAction(super::Action::Write(super::Written::ResetTuner {
                        id: id.clone(),
                        name: url.clone(),
                    })),
                ),
            );
            held = held.push(
                button(prose(strings::lookup(Text::TunersDelete), typeface::BODY)).on_press(
                    Message::DashboardAction(super::Action::Ask(
                        crate::screen::confirm::Pending::of(
                            crate::screen::confirm::Destructive::DeleteTuner { id },
                            url,
                        ),
                    )),
                ),
            );
        }
        page = page.push(held);
    }

    if read_only {
        return page;
    }

    page = page.push(
        button(prose(strings::lookup(Text::TunersDiscover), typeface::BODY)).on_press(
            Message::DashboardAction(super::Action::Write(super::Written::DiscoverTuners)),
        ),
    );
    for found in &state.discovered {
        page = page.push(prose(found.url.clone().unwrap_or_default(), typeface::BODY));
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
            button(prose(strings::lookup(Text::TunersAdd), typeface::BODY)).on_press(
                Message::DashboardAction(super::Action::Write(super::Written::AddTuner {
                    url: state.tuner_url.clone(),
                    kind: state.tuner_type.clone(),
                }))
            ),
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    )
}

fn providers<'a>(mut page: Page<'a>, state: &'a State, read_only: bool) -> Page<'a> {
    page = page.push(prose(
        strings::lookup(Text::ProvidersTitle),
        typeface::HEADING_2,
    ));

    for provider in &state.providers {
        let Some(id) = provider.id.clone() else {
            continue;
        };
        let named = provider.type_.clone().unwrap_or_default();
        let mut held =
            row![prose(named.clone(), typeface::BODY)].spacing(style::drawn(space::GUTTER.drawn()));
        if !read_only {
            held = held.push(
                button(prose(
                    strings::lookup(Text::ProvidersDelete),
                    typeface::BODY,
                ))
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::DeleteProvider { id },
                        named,
                    ),
                ))),
            );
        }
        page = page.push(held);
    }

    if read_only {
        return page;
    }

    page = page.push(
        row![
            button(prose(
                strings::lookup(Text::ProvidersSchedulesDirect),
                typeface::BODY
            ))
            .on_press(Message::DashboardAction(super::Action::ProviderKind(true))),
            button(prose(strings::lookup(Text::ProvidersXmltv), typeface::BODY))
                .on_press(Message::DashboardAction(super::Action::ProviderKind(false))),
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    );

    if state.provider.schedules_direct {
        page = page
            .push(
                text_input(
                    strings::lookup(Text::ProvidersUsername),
                    &state.provider.username,
                )
                .style(style::input)
                .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
            )
            .push(
                text_input(
                    strings::lookup(Text::ProvidersPassword),
                    &state.provider.password,
                )
                .style(style::input)
                .secure(true)
                .on_input(|typed| Message::DashboardAction(super::Action::TypedPassword(typed))),
            )
            .push(pick_list(
                state.countries.clone(),
                Some(state.provider.country.clone()),
                |chosen| Message::DashboardAction(super::Action::ProviderCountry(chosen)),
            ))
            .push(
                text_input(
                    strings::lookup(Text::ProvidersPostcode),
                    &state.provider.postcode,
                )
                .style(style::input)
                .on_input(|typed| Message::DashboardAction(super::Action::ProviderPostcode(typed))),
            )
            .push(
                button(prose(
                    strings::lookup(Text::ProvidersLineup),
                    typeface::BODY,
                ))
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::FetchLineups,
                ))),
            );
        for lineup in &state.lineups {
            page = page.push(
                button(prose(
                    lineup.name.clone().unwrap_or_default(),
                    typeface::BODY,
                ))
                .on_press(Message::DashboardAction(
                    super::Action::ProviderLineup(lineup.id.clone().unwrap_or_default()),
                )),
            );
        }
    } else {
        page = page.push(
            text_input(strings::lookup(Text::ProvidersPath), &state.provider.path)
                .style(style::input)
                .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
        );
    }

    page.push(
        button(prose(strings::lookup(Text::ProvidersAdd), typeface::BODY)).on_press(
            Message::DashboardAction(super::Action::Write(super::Written::AddProvider)),
        ),
    )
}

fn mapping<'a>(mut page: Page<'a>, state: &'a State, read_only: bool) -> Page<'a> {
    page = page.push(prose(
        strings::lookup(Text::MappingTitle),
        typeface::HEADING_2,
    ));
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
        .spacing(style::drawn(space::GUTTER.drawn()));
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
        page = page.push(held);
    }
    page
}

fn dvr<'a>(mut page: Page<'a>, state: &'a State, read_only: bool) -> Page<'a> {
    page = page.push(prose(strings::lookup(Text::DvrTitle), typeface::HEADING_2));
    for field in DVR {
        page = page.push(super::control(*field, state.dvr.value(*field), false));
    }
    if !read_only {
        page = page.push(
            button(prose(strings::lookup(Text::DashboardSave), typeface::BODY))
                .on_press(Message::DashboardAction(super::Action::Save)),
        );
    }
    page
}
