use crate::ui_state::{LibraryView, Pane, UiState, fade_color};
use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::{StatefulWidget, Widget},
};

pub struct BreadCrumbs;

impl StatefulWidget for BreadCrumbs {
    type State = UiState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let theme = &state.theme_manager.get_display_theme(true);
        let (top_level, top_count) = state.get_sidebar_details();

        let dimmed_secondary = fade_color(theme.dark, theme.text_secondary, 0.80);

        let spans = match state.get_pane() {
            Pane::SideBar => {
                vec![Span::from(format!("{top_level} ({top_count})")).fg(theme.text_muted)]
            }
            Pane::TrackList => match top_level {
                LibraryView::Albums => {
                    let Some(album) = state.get_selected_album() else {
                        return;
                    };
                    Vec::from([
                        Span::from(format!("{top_level}  ")).fg(theme.text_muted),
                        Span::from(format!("{}", album.title)).fg(dimmed_secondary),
                        Span::from(format!(" [{}]", album.artist)).fg(theme.text_muted),
                    ])
                }
                LibraryView::Playlists => {
                    let Some(playlist) = state.get_selected_playlist() else {
                        return;
                    };
                    Vec::from([
                        Span::from(format!("{top_level}  ")).fg(theme.text_muted),
                        Span::from(format!("{}", playlist.name)).fg(dimmed_secondary),
                    ])
                }
            },
            _ => vec![],
        };

        Line::from(spans).render(area, buf);
    }
}
