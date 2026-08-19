# Jellium Web verification

Run against a Jellyfin server holding the media each section names, in current
Firefox and current Chromium on desktop Linux, with the one Jellium Web bundle,
which carries both the WebGPU and the WebGL2 backend and chooses between them
in the browser.

Start the local server with `jellium-cli web`, sign in, and follow each section
in order. A section passes only when every line in it holds.

## Deployment

- A Jellyfin server at a path prefix, such as `https://host/jellyfin`, direct
  plays and transcodes, and every relayed url keeps the prefix.
- A Jellyfin server behind a proxy that gzips text responses relays playlists,
  and HLS plays.
- The browser's network panel shows item, search and image responses arriving
  gzip-encoded, and playlist responses arriving unencoded.
- Ctrl-C exits while a title is playing, and the transcode job is gone from the
  Jellyfin dashboard.

## Direct play

- A movie in H.264/AAC MP4 plays, and the display says it is direct playing.
- No transcode job appears in the Jellyfin dashboard while it plays.
- Ten minutes of continuous play produces no stall, and the local server's
  resident memory ends where it started.
- The first frame appears within 2 seconds of pressing Play against a server on
  the local network.
- A song in FLAC, Ogg Vorbis, Opus or WAV direct plays, and the display says
  so.

## Transcode

- A title in a codec the browser cannot decode plays, and the display says it
  is transcoding.
- The same title on a browser that decodes it direct-plays instead.
- Leaving the player ends the transcode job on the Jellyfin dashboard.

## HLS

- A transcoded title loads through hls.js and plays.
- The browser's network panel shows every media request going to the local
  server's origin and none anywhere else.
- No playlist body, segment url or subtitle url carries `api_key`, `ApiKey` or
  `X-Emby-Token`.

## Subtitles

- An SRT, ASS, SSA or SUB stream plays as a native text track, and selecting it
  does not start a transcode.
- A PGS, VOBSUB or DVDSUB stream plays burned in, and the display says the
  transcode is for the subtitles.
- Turning subtitles off restores direct play on the next playback.
- The playback settings screen offers the subtitle modes in the reference's
  order: Default, Smart, Only forced, Always, None.
- The audio and subtitle tracks selected by default match the user's
  server-side language and subtitle-mode configuration.

## Seeking

- Seeking a direct-play title issues range requests and plays from the new
  position without refetching from the start.
- Seeking a transcoded title resumes at the position sought.
- A seek shows the new position within 1 second.

## Queue

- Playing an episode queues the rest of its season and advances with no
  countdown when one ends.
- Play All queues a series, a season, an album and an artist.
- Instant Mix on a song, an album and an artist queues from the Jellyfin
  server.
- The queue view opens from the now-playing bar, lists what is upcoming,
  removes an item, and its back control returns to what was open with playback
  undisturbed.
- An item removed from the queue stays gone after shuffle is toggled on and off
  again.
- Shuffle reshuffles only what has not played, and toggling it off restores the
  original order.
- Repeat offers off, one and all, from the now-playing bar and from the video
  display's settings menu, for audio and for video.

## Controls and keys

- The video display offers play, scrub with a buffered indicator and a tick per
  chapter, elapsed and total time, skip back 10 and forward 30, volume with
  mute, audio, subtitle and quality selection, version selection, previous and
  next while the queue holds more than one item, the chapter list and
  fullscreen.
- The audio controls offer play, scrub, elapsed time, previous and next
  whatever the queue holds, stop, volume with mute, repeat, shuffle, favourite,
  the queue view, cast, and watch-together where the server grants SyncPlay,
  and no subtitle, quality or fullscreen control.
- Space, `k`, the arrows, `f`, `m`, `n`, `p` and Escape drive the player.
- One drag of the scrub bar produces one seek and one progress report.
- Every control answers within 100 milliseconds.
- The chosen quality, the volume and the mute state survive a browser restart.

## Full screen and the overlay

- Fullscreen works and the display stays visible and usable in it.
- The display and the cursor hide after 3 seconds without input while playing,
  and stay while paused or while a menu is open.
- Video fills the viewport with the browser's native controls absent.
- Audio plays with a now-playing bar that survives navigation between screens,
  and starting video stops audio that was already playing.

## Media keys

- The operating system's play, pause, previous, next and seek media keys drive
  playback.
- The system media controls show the title, the series or artist, and artwork.
- Jellium Web appears in another Jellyfin client's device picker while a tab is
  connected, and is gone from it once the last tab has closed.

## Session lifecycle

- Start, progress on pause, unpause, seek and track change and every 10 seconds
  thereafter including while paused, and a stop carrying the final position all
  reach the Jellyfin server.
- Returning to an item's detail after playing it shows the new position and
  played mark without a manual reload.
- The home screen shows the next episode in next-up with the finished one gone
  from continue-watching.
- Audio keeps playing and reporting with its tab hidden, and video is not
  auto-paused when its tab is hidden.
- A paused tab left hidden for ten minutes keeps its session, and playback
  resumes there when the tab returns.
- No transcode job survives playback ending by the stop control, by leaving the
  player, by navigating away, by closing the tab, or by the local server
  exiting.
- A second tab that starts playback ends the first, and the first says why.
- Closing the tab in the instant after the queue advances reports the new
  item's position, not the finished one's.
- Two tabs pressing Play at the same instant leave one playing, and the other
  says another tab took the session rather than starting playback again.

## Failures

- Each of no playable source, a refused transcode, an item with no media
  source, a decode error, a dropped stream and an unreachable server produces
  on-screen text naming that cause.
- A decode error is retried once with direct play disabled before it is
  reported.
- A dropped stream is resumed once at the last reported position before it is
  reported.
- A rejected access token during playback stops playback and returns to the
  login screen.

## Timing

- The first frame of a direct-play title appears within 2 seconds.
- A seek shows the new position within 1 second.
- The display's controls answer within 100 milliseconds.
- Browsing, image loading and search stay responsive while a stream is
  relayed.

## Live refresh

- A title marked played in a second real client changes on the home rows, the
  library grid, item detail, search results and the queue within 1 second,
  without a reload.
- A title favourited in that client changes the same way.
- The scroll position, the sort order and the paging do not move, and the item
  playing is undisturbed.
- Finishing an episode there removes it from continue-watching here and shows
  the next one in next-up.
- Restarting the Jellyfin server produces text naming that cause, distinct from
  the text an unreachable server produces.
- Stopping the local server's link to Jellyfin shows the live-updates
  indicator, and browsing, search, detail, playback, mark-played and
  mark-favourite all keep working while it is shown.
- Restoring the link clears the indicator and the screen shown is current.

## Being controlled

- A real Jellyfin client lists Jellium Web in its device picker while a tab is
  open, and stops listing it once the last tab closes.
- Play from that client starts playback here within 500 milliseconds; pause,
  unpause, seek, next, previous, stop, volume, mute, repeat and shuffle each
  take effect here.
- Audio track, subtitle track, quality, fullscreen and the display toggle each
  take effect here.
- Go home, go to search and display content each move this client, and a
  message shows its header and text and then stops showing.
- With four tabs open, one command produces one effect, in one tab.
- A command sent while nothing is playing changes nothing on screen.
- A command sent by a second Jellyfin user is obeyed.

## Remote mode

- The device picker opens from the player display, from the now-playing bar
  and from `/remote`, and lists real sessions by device name with client name
  beneath, this client's own absent.
- Choosing a target while something plays here sends it the current item, the
  rest of the queue and the position, and stops playback here.
- Choosing a target from `/remote` with nothing playing here sends it nothing.
- Play, Play All and instant mix all reach the target while the mode is
  active.
- The panel's play/pause, stop, scrub, skip back, skip forward, next,
  previous, volume, mute, repeat and shuffle each drive the target, and its
  now-playing item, position, paused, muted, volume and repeat state follow the
  target without the user acting.
- A second tab entering the mode takes it, and the first says so.
- Reloading the page returns to local control, with the target still playing.
- Closing the target's client ends the mode with text naming that cause.
- A Play sent to this client while the mode is active plays here and leaves the
  target playing.

## SyncPlay

- `/syncplay` lists the groups a second real client offers, with their names,
  participants and state, and the list refreshes while the screen is open.
- Creating a group while a title plays here hands the group that title, the rest
  of the queue and the position, and the group is named for the title.
- Joining a group that is already playing has this client playing at the group's
  position within 3 seconds.
- Play, pause, seek, next, previous, choosing a queue item, removing a queue
  item, repeat and shuffle each take effect for every member.
- Volume, mute, fullscreen, audio track, subtitle track and quality change
  nothing for the other members.
- Escape closes the player, stops the group, and leaves this client a member of
  an idle group whose queue is intact.
- Leaving the group stops playback for no other member.
- A second tab joining takes the group from the first, which says so once.
- Entering a group ends remote mode, and taking a remote target leaves the
  group, each saying so.
- Reloading the page returns to local mode, and the group no longer lists this
  client.
- Stopping the local server's link to Jellyfin pauses playback here, keeps
  membership, and says so; restoring it rejoins the group's schedule.
- A user whose SyncPlay access is None sees no SyncPlay control anywhere, and a
  user whose access is JoinGroups sees no create action.

## Two clients in one group

- Two Jellium Web tabs in one group, on one machine, stay within 400
  milliseconds of each other through ten minutes of play, a pause, an unpause
  and three seeks.
- A group shared with real jellyfin-web stays in step, joined from each side in
  turn, and both clients obey next, previous and a queue item chosen from
  either.
- Running the local server with `--allow-remote` and a browser on another
  machine changes none of the above and produces no additional warning.

## Live TV

- Home carries an on-now row of favourite channels first and then channels in
  number order, each showing its logo, number and current programme with an
  elapsed bar, and home paints before the row arrives.
- An on-now card whose current programme carries a timer draws the record glyph
  on the top trailing corner of its image: filled and red for a single timer,
  and the faded series glyph for a series timer.
- A server with no Live TV service, and an account denied Live TV, each show no
  Live TV entry, no on-now row and no Record anywhere, and reaching `/livetv`
  by hand names the cause.
- `/livetv` opens on the Guide with five tabs: Guide, Channels, Recordings,
  Schedule and Series.
- (tuner) The guide opens at the current half hour showing two hours, ruled
  every 30 minutes, with a marker at the present instant that advances while it
  is open.
- (tuner) Earlier and Later move one screen; a date inside the reported range
  is reached and one outside it names the cause.
- (tuner) Left, right, up, down and Enter move the guide and open the focused
  cell.
- (tuner) A cell airing now plays its channel; any other cell opens programme
  detail.
- (tuner) The Channels tab lists channels in number order with favourites
  first, filters TV from radio, and favouriting a channel moves it.
- (tuner) On a phone the Channels tab draws a more_vert on the trailing edge of
  each card's footer; pressing it raises a sheet offering Play, and choosing
  Play tunes the channel.
- (tuner) On a phone a channel's card opens the channel's own detail page,
  which offers the favourite control.
- (tuner) A radio channel plays through the now-playing bar with no player
  view.
- (tuner) Programme detail shows the title, channel by name and number, start
  and end, overview, image, genres and flags, with Record, Record Series,
  Cancel and Play as its state allows.

## Live playback

- (tuner) Selecting a channel shows a tuning indicator naming it, and the first
  frame appears within 8 seconds against a server on the local network.
- (tuner) The display shows a LIVE badge, the channel's name and number, and
  the current programme's title, times and elapsed bar, with no scrub bar.
- (tuner) Pause pauses; unpause resumes at the live edge and says so.
- (tuner) Next and previous change channel; skip back, skip forward, the
  chapter list and the queue are absent.
- (tuner) Audio track and quality selection are offered; subtitle and version
  selection are not.
- (tuner) Record creates a timer and the display then says the programme is
  being recorded.
- (tuner) At a programme boundary the display advances without interrupting
  playback.
- (tuner) Leaving the player frees the tuner at once, seen in the Jellyfin
  dashboard.
- (tuner) A live playback left paused for five minutes stops and names the
  cause, and the tuner is free.
- (tuner) Watching a channel while every other tuner is busy names that no
  tuner is free, without retrying and without naming another session.

## Recordings and timers

- (tuner) The Recordings tab lists recordings newest first with in-progress
  ones at the top and marked.
- (tuner) A completed recording plays with an ordinary resume position, and
  offers Delete behind a confirmation and no stop.
- (tuner) An in-progress recording plays as a partial file, and offers Stop
  Recording and no delete.
- (tuner) Record on a programme creates a timer with no prompt; Record Series
  prompts with the server's defaults and creates on confirmation.
- (tuner) The Schedule tab lists timers by start time grouped by day with
  channel, programme, time and status, shows a conflicted timer as conflicted,
  and cancels one.
- (tuner) The Series tab lists series timers by name, edits every field the
  server carries, and cancels one.
- (tuner) On a phone the Recordings tab draws a play_arrow at the bottom
  trailing corner of each recording's image, and pressing it plays the
  recording.
- (tuner) On a phone a recording's card opens its detail page, whose more
  control raises a sheet offering Delete media; choosing it asks for the
  recording's name and deletes it only once the name matches.
- (tuner) On a phone the Schedule tab draws a more_vert at the bottom trailing
  corner of each in-progress recording's image; the sheet it raises offers
  Cancel recording, and confirming stops the recording.
- (tuner) On a phone a scheduled timer's card opens the programme page it names
  with Cancel offered, and a timer naming no programme opens recording options
  carrying Cancel recording.
- (tuner) On a phone the Series tab draws a more_vert on each card; the sheet
  offers Cancel series, and confirming removes the series timer.
- (tuner) On a phone a series timer's card opens its own options, which save
  every field.
- (tuner) A recording written by a series timer draws fiber_smart_record in
  #cb272a on its image, one written by a single timer draws
  fiber_manual_record in #cb272a, and one the server reports no timer for draws
  no glyph.
- (tuner) A scheduled timer belonging to a series timer draws
  fiber_smart_record in #cb272a on its Schedule card, and every Series tab card
  draws that same glyph in that same colour.
- (tuner) A channel's card on the Channels tab draws no record glyph, whatever
  timer covers the programme on it now.
- (tuner) A timer created or cancelled in a second real client updates the
  Schedule tab, the Series tab and the guide's record markers within 1 second,
  with the scroll position and the order unmoved.

## Windowed surfaces

- (tuner) The guide over 500 channels and 14 days shows its first screen within
  500 milliseconds and holds 30 frames per second scrolling either axis, with
  the browser's memory ending where it started after ten minutes of scrolling.
- (tuner) The channel list over 500 channels holds 30 frames per second.
- The queue, Recordings, Schedule and Series lists scroll without a paging
  control, and none of them offers a sort control.
- The library grid and search results keep their paging controls.
- The browser's network panel shows one programme query per screenful of guide
  and none while the guide sits still.

## Dashboard

- An administrator reaches the dashboard from the chrome and leaves it with
  Back; a non-administrator has no dashboard control anywhere, and reaching a
  dashboard route by hand shows nothing administrative.
- A dashboard route replaces the browse chrome with a navigation column, keeps
  the now-playing bar, and leaves audio playing across every dashboard screen.
- Removing the signed-in user's administrator status in a second real client
  removes the dashboard control and returns the user to home from any dashboard
  screen within the coalescing window, naming that cause, and raises no
  unsaved-edit warning.
- Asking for a relayed route as a non-administrator is refused by Jellyfin and
  that refusal is shown as text; the local server applies no check of its own.
- Dashboard home shows the server's name and version, every session on the
  server including this client's own with its device, client, user and what it
  plays, each running task with its progress, and a scan indicator distinct
  from the task indicators, all updating without acting.
- Dashboard home offers a global scan, and Restart and Shutdown each behind a
  confirmation naming the action.
- A restart started from the dashboard shows text naming that an administrator
  restarted the server, told apart from an unreachable server and from
  `ServerStopping`, and the client reconnects on the existing backoff without
  returning to the login screen; a shutdown shows its own text and attempts no
  reconnection.
- `jellium-cli web` neither stops nor exits when the Jellyfin server restarts or
  shuts down.
- A banner stands while the server reports a restart is required.
- A task screen writes the task's name once and no collection title over it.

## Server configuration

- Configuration pages exist for general settings, networking, branding,
  playback resume, streaming, transcoding and trickplay, each saving only on an
  explicit action.
- Saving any configuration page leaves every field no control covers exactly as
  the server answered it, checked by reading the section back over the API.
- Leaving a form holding unsaved edits warns and names what is lost; proceeding
  discards them and staying keeps them.
- Reading the encoding, network, metadata, trickplay and Live TV sections
  through `jellyfin-api` succeeds against a real server.
- The networking page stands each of its five groups under its own heading,
  with the reference's own room under each group and between its controls.

## Users

- The user list shows every user, and creates and deletes one.
- A user screen heads with that user's own name, at the size a section title
  is written at, and the add-user screen heads "Add User".
- A user screen carries profile, access, parental control and password; the
  profile panel holds that user's policy and the access panel holds the
  library, channel and device lists.
- A user's password is set and reset from the user screen, and a user's image is
  removed.
- Saving a user policy leaves every field no control covers as the server
  answered it.
- Library, channel and device access are chosen from checkboxes naming what
  the server holds, each list standing only while its own "all" box is clear,
  and a device reads as its name joined to its application's.
- A server holding no channels shows no channel access group.
- Deleting the signed-in user's own account and removing their own
  administrator status are absent, with text naming that cause where each would
  stand.
- Every control on a user's profile, access and parental panels carries the
  sentence the reference writes beside it, and no raw configuration key appears
  anywhere on those panels.
- A user's bitrate limit is typed in Mbps and reads back in Mbps after a save.
- Editing another account shows an administrator checkbox; editing one's own
  shows the sentence naming why it is absent, and the rest of the panel is
  unchanged.

## Libraries

- A library screen heads with that library's own name and writes "Folders"
  over its media paths as a heading rather than as body text.
- A virtual folder is created, renamed and removed; a media path is added and
  removed; choosing a path browses the server's own filesystem rather than
  taking typed text.
- The library options form saves without disturbing the fields no control
  covers.
- The first group of the library options form is headed "Library Settings".
- Subtitle download languages are chosen from checkboxes naming every language
  the server reports.
- Metadata readers and subtitle downloaders stand as ranked rows, each moved
  one place by the control on its row, and a subtitle downloader is turned on
  and off by the box on its row.
- Saving a library's options writes the ranked orders the rows stand in and
  the downloaders left off.
- A scan starts for all libraries and for one named library, and
  `RefreshProgress` updates the named library's progress in place; an event
  naming an item no open screen shows changes nothing.
- Item detail offers Refresh Metadata with its replace and scan mode options to
  an administrator and to nobody else.
- Every control on a library's options carries the sentence the reference
  writes beside it, and the options stand in three headed sections, each with
  the room the reference leaves under a section.
- The libraries page writes no heading of its own and stands its content two
  steps down the page.

## Scheduled tasks

- The task list shows every task with its state and its running progress, and a
  task starts.
- A running task stops behind a confirmation naming it.
- Task detail shows the last execution's status and duration.
- A task's triggers are added and removed and written back as the whole list.
- `ScheduledTasksInfo` updates the task list in place, and a progress change
  appears within 1 second.
- The task list stands clear of the top of the page, and on a wide window its
  categories stop short of the right edge rather than running to it.

## Logs and activity

- The logs screen lists the server's log files with their names and sizes.
- Opening a log shows its last 2 MiB and states that it is a tail, naming the
  file's full size; the browser's network panel shows no more than 2 MiB
  delivered.
- A log file the server does not hold produces text naming it.
- In an open log, the file's name, the sentence about the tail and the panel
  holding the lines stand closer together than the rows of the logs screen do.
- The activity log builds widgets only for the entries shown, fetches a page at
  a time as the window moves, shows each entry's time, name, short overview,
  type and user, and filters by whether an entry names a user.
- `ActivityLogEntry` prepends in place under the existing coalescing rule
  without moving the scroll position.

## Plugins

- The installed-plugin list shows each plugin's name, version and status, and
  enables, disables and uninstalls one, uninstall behind a confirmation naming
  it.
- The catalog lists every package the configured repositories offer with its
  name, description and versions; installing is behind a confirmation naming the
  package, the version and the repository and stating that the server executes
  it; a running install is cancelled.
- The five package messages update the catalog and the installed list in place.
- The repository list shows each repository's name and url, removes one, and
  adds one behind a confirmation naming the url.
- No image the dashboard renders is fetched from any origin but the local
  server's own, checked in the browser's network panel.

## Plugin configuration pages

- (plugin) A plugin's screen lists the configuration pages that plugin hosts; a
  page belonging to no installed plugin is not listed, and a plugin hosting no
  page shows no page and no empty frame.
- (plugin) A configuration page renders in a frame whose origin is opaque, so it
  carries no session cookie and issues no relayed request of its own, and the
  frame occupies the viewport beneath the dashboard chrome with leaving always
  possible.
- (plugin) The frame reaches the application through exactly nine verbs; a page
  naming a plugin id reaches no other plugin's configuration; a verb outside the
  set produces text naming the verb.
- (plugin) The local server serves a configuration page only under a name it has
  itself seen in a configuration-page listing during this run, and refuses any
  other name by name.
- (plugin) Every subresource of a served page resolves to a relayed same-origin
  path, and a page needing one from anywhere else produces text naming that
  cause.
- (plugin) Reading and writing a plugin's configuration preserves the plugin's
  own fields, and a configuration over 64 KiB is refused with text naming the
  size and the cap.
- (plugin) A configuration page whose inline script body or comment spells
  `src="` is served with those bytes unchanged.
- (plugin) A frame whose tab dies without releasing its grant loses the grant
  within fifteen minutes, and the page's path then refuses.
- The overlay hosts more than one element at a time, each with its own identity,
  stacking, pointer behaviour and message channel; video playback, audio
  playback, the on-screen display, subtitles and full screen behave as they did
  before.

## Devices and API keys

- The device list shows each device's name, client, user and last-seen time,
  sets a custom name, and deletes one behind a confirmation naming it.
- Confirming deletion of this installation's own device states that it ends the
  session, and afterwards the client returns to the login screen naming that
  cause.
- The API key list shows each key's application name and creation date, creates
  one, and revokes one behind a confirmation naming it.

## Live TV administration

- (tuner) Tuner hosts are listed, added, deleted, discovered and reset.
- (tuner) A listing provider is added and deleted as Schedules Direct with its
  credentials, country, postcode and a lineup the server reports, or as XMLTV
  with its path.
- The Schedules Direct country list parses.
- (tuner) A tuner's channels are mapped to a provider's.
- The DVR settings page saves without disturbing the fields no control covers.
- Every control on the DVR settings page carries the sentence the reference
  writes beside it, drawn as MUI's outlined fields.
- Recording padding is typed in minutes and reads back in minutes after a save,
  and the server holds it in seconds.
- The two recording padding fields write "minutes before" and "minutes after"
  inside the field, against its trailing edge, and neither carries a line of
  text beneath it.

## Read-only mode

- `jellium-cli web --read-only` starts, and under it logout, SyncPlay group
  creation and every group verb, remote mode, marking played and marking
  favourite are absent rather than disabled.
- Under it, playback negotiation and the start, progress and stopped reports
  work unchanged.
- One persistent, non-blocking indicator states the mode on every screen, and no
  action carries an explanation of its own.
- A write route asked for directly under read-only is refused before the request
  body is read and before the Jellyfin server is reached.
- Starting with `--allow-remote` and without `--read-only` warns that server
  administration, including user deletion and plugin installation, is reachable
  from the network and names `--read-only`; starting with both warns that the
  reachable surface is read-only.
- A `Report` frame hand-sent over the event socket under read-only — group
  creation, a group join, a group verb, a take of remote mode, a drive — is
  refused as read-only by the local server and reaches the Jellyfin server not
  at all.

## Dashboard under load

- An activity log of 100,000 entries shows a first screen within 500
  milliseconds and holds 30 frames per second while scrolled, with the browser's
  memory ending where it started after ten minutes of scrolling.
- A 2 MiB log tail shows a first screen within 500 milliseconds and holds 30
  frames per second while scrolled.
- A catalog of 500 packages shows a first screen within 500 milliseconds and
  holds 30 frames per second while scrolled.
- A burst of 1,000 activity entries produces one coalesced refresh and grows
  neither the local server's nor the browser's memory without bound.
- Every dashboard screen, the plugin configuration frame and read-only mode work
  in current Firefox and current Chromium on desktop Linux.
- No byte the local server delivers to the browser for the dashboard carries the
  access token the local server holds, checked in the browser's network panel.

## Login surface

- With one saved server holding a credential, launching signs straight in,
  however many other servers are saved.
- With saved servers and no credential on the active one, launching opens the
  server list; with none saved, it opens add-server and nothing else.
- Adding `host:8096` with no scheme stores the url that answered, and the
  network panel shows `https://` tried before `http://`.
- Adding a url that normalizes to a saved one selects that entry and leaves the
  list one entry long.
- Adding an unreachable server and a server below 10.10.0 each leave the typed
  text on screen with different sentences, and the list is unchanged.
- Adding a server that has not been set up walks into the wizard, and its entry
  appears in the list only after the wizard's sign-in.
- A server whose name changed on the server shows the new name after the next
  successful select, and an unreachable server keeps the name it last reported.
- A server that has never answered shows its url alone.
- Selecting an unreachable saved server leaves the list on screen with the cause
  against that entry, and its credential intact.
- Revoking the active server's token on the Jellyfin dashboard, then launching,
  lands on that server's login screen naming the rejected sign-in, and the
  entry is still saved with no credential.
- Switch while a title is playing, while in a SyncPlay group and while driving
  a remote target each ask for confirmation naming what ends, and confirming
  ends all three.
- After a switch, `jellium-cli users me` names the account on the newly active
  server, and the server left is re-entered without a password.
- Removing an entry that holds a credential takes its session off the Jellyfin
  server's device list before the entry disappears.
- Removing the active server signs out and lands on the list, or on add-server
  when nothing is left.
- The picker lists the server's public users; picking one fills the name and
  still asks for a password, and an account with no password signs in on an
  empty one.
- A user's picture appears in the picker, and a request for a user id outside
  the public list is refused.
- The login page shows the user picker alone under Manual login, or the typed
  name and password alone over Cancel, and never both; Manual login and Cancel
  move between them.
- A server with no public users opens on the typed name and password, with
  neither Manual login nor Cancel anywhere on the page.
- (real server) A Quick Connect code shown here is authorized on a second real
  device, and this browser is signed in within five seconds of the
  authorization.
- The Quick Connect screen never shows a secret, and no response in the
  network panel across the whole flow carries one.
- Letting a code expire offers one action that gets a new code; turning Quick
  Connect off on the server ends the flow with different text and takes the
  option off the login screen.
- Quick Connect names Jellium Web and this installation's device on the
  authorizing device.
- (real server) A password reset writes a real pin file: the screen states the
  file was written, shows the server's path quoted, shows the expiry and takes
  the pin; redeeming it names the accounts whose password is now unset.
- The contact-an-administrator and in-network-required answers each show their
  own sentence and no pin field.
- A pin the server refuses is named on screen and leaves the field usable.
- Opening a second tab on a different server's login screen refuses the first
  tab's next login-stage request, naming that another tab moved.
- The network panel shows no websocket opened while any login screen is shown.
- Under `--read-only`, adding, selecting and switching work, Remove is absent
  on credentialed entries and present on the rest, Quick Connect sign-in works,
  and password reset is absent and its endpoints refuse.
- `jellium-cli web --bind 0.0.0.0 --allow-remote --advertise <host>` warns that
  the saved server list and every sign-in path are reachable from the network.
- Every screen here works in current Firefox and current Chromium on desktop
  Linux.

## Setup wizard

- (fresh) Submitting the login screen against a server that has not been set up
  opens the wizard on step one, whatever was typed as the user name and
  password.
- (fresh) Reloading the browser on any step resumes on that step with every
  value the earlier steps stored still present.
- (fresh) The browser's network panel shows no request to the local server
  opening a websocket while the wizard is open.
- (fresh) Step one sets the server language from the list the server reports and
  sets the server name, and states that Jellium Web's own language is
  unaffected.
- (fresh) Step two blocks Next on an empty name and on a confirmation that
  differs, and accepts an empty password.
- (fresh) Returning to step two with a different name leaves the server holding
  one user under the new name.
- (fresh) Step three creates a library by name, content type and paths chosen by
  browsing the server's filesystem, renames it, removes it, and completes with
  none configured.
- (fresh) The content types step three offers are the ones the dashboard's
  library administration offers.
- (fresh) Step four sets the metadata language and country from the lists the
  server reports.
- (fresh) Step five sets remote access and automatic port mapping, and both are
  set on the server afterwards.
- (fresh) Finishing signs in as the administrator just created, lands on home
  with the event socket open, and `jellium-cli users me` names the same user.
- (fresh) A wizard step the server refuses keeps the step's values, does not
  advance, and shows the server's own message beneath the client's sentence.
- (fresh) Back on step one returns to the login screen, and a second server url
  can be entered.
- (fresh) `jellium-cli web --read-only` against a server that has not been set
  up offers no wizard and names that cause.
- (fresh) `jellium-cli web --bind 0.0.0.0 --allow-remote --advertise <host>`
  warns that first-run setup and first-administrator creation are reachable from
  the network, and the wizard works from another machine.
- (fresh) A server below 10.10.0 that has not been set up is refused by version
  before the wizard opens.
- (fresh) A second tab opened mid-wizard shows each step's stored values, and
  finishing in one tab leaves both signed in.
- (fresh) The whole wizard works in current Firefox and current Chromium.
- (fresh) After finishing, the server's device list holds one entry for this
  installation, naming Jellium Web and the same device `jellium-cli` reports,
  and no entry without a device name.
- No byte the local server delivers to the browser during the wizard carries an
  access token, checked in the browser's network panel.

## Settings

1. Sign in and open Settings from the chrome. Every screen is reachable, the
   now-playing bar stays while audio plays, and Back returns to what was open.
2. On Profile, choose an image over 4 MiB: it is refused with its size and the
   cap and nothing is sent. Choose a `.txt` renamed `.jpg`: it is refused by
   type. Choose a real JPEG under the cap: the image appears here and on the
   dashboard's user screen without a reload.
3. Remove the image behind its confirmation, and confirm it is gone from both
   screens.
4. On Password, change the password with the wrong current password: the text
   names that cause. Change it with the right one, then confirm this browser is
   still signed in and `jellium-cli users me` still works with the shared
   session. Confirm another device signed in as this account is signed out.
5. Play an item with a real text subtitle stream. Change text size, text colour,
   background colour, opacity and drop shadow, and confirm the cues take each
   change in the player view and in full screen.
6. Sign in to the same account from another browser and confirm the bitrate
   ceiling, skip lengths, per-library sort, subtitle appearance and home row
   toggles arrived, and that volume and mute did not.
7. On another device, start a Quick Connect sign-in and authorize its code here
   behind the confirmation. Authorize the same code again, then a code that was
   never issued, and confirm the three texts differ.
8. Run with `--read-only`: every settings screen reads, no save, password
   change, upload, removal or authorize control stands, and the on-screen
   display's quality selector still changes quality for the run.

## Browsing breadth

1. Open a movie library and confirm the Suggestions, Favorites, Genres and
   Studios tabs; a television library for Suggestions, Favorites, Upcoming,
   Episodes, Genres and Networks; a music library for Suggestions, Favorites,
   Album Artists, Artists, Songs and Genres.
2. Confirm no library screen offers a Collections or a Playlists tab, and that
   both are entries in the home screen's library list.
3. Select a genre, a studio, a network, an artist and an album artist, and
   confirm each opens that value's items. Select a cast name on item detail and
   confirm it opens that person's.
4. Watch the local server's request log while doing so and confirm no path
   carries a genre, studio, person or artist name.

## The windowed grid under load

1. Point at a library holding at least 50,000 items. Confirm the first screen
   is drawn within 500 milliseconds.
2. Scroll continuously for ten minutes. Confirm the frame rate holds at or above
   30 frames per second and the browser's memory ends where it started.
3. Confirm no browse surface carries a page number, a next control or a previous
   control.
4. Change a library's sort and change it back; confirm the window is where it
   was. Sign in from a second browser and confirm the sort arrived and the
   filters did not. Press Back and confirm the list returns with the filters it
   was left with.
5. Open the filter surface, apply several filters, and confirm it states how
   many are active and clears them all in one action.

## The letter jump

1. On a name-sorted list, press `M` and confirm the first item at or after `M`
   is at the top of the window.
2. Confirm the strip is absent on every sort that is not a name sort.

## Trickplay and chapter thumbnails

1. Play an item the server has built trickplay for. Hover the scrub bar and
   confirm the frame at that position appears within 100 milliseconds of the
   pointer settling. Drag and confirm the frame follows.
2. Select another version and confirm the preview still draws.
3. Play an item with chapter images but no trickplay and confirm the chapter
   image stands in. Play one with neither and confirm the bar is otherwise
   unchanged.
4. Open the on-screen display's chapter list and confirm each name carries a
   thumbnail. Confirm neither the now-playing bar's scrub nor the remote scrub
   shows a preview.

## Identify and remote images

1. As an administrator, open the metadata manager from item detail. Confirm
   every other user finds no entry point.
2. Edit a field, save, and confirm every field no control covers survived, by
   reading the item back from the Jellyfin server directly.
3. Identify against a provider, apply a candidate, and confirm the images
   arrived. Confirm the browser's network log carries no provider url — every
   poster is fetched from this origin under `/foreign`.
4. Upload an image that is not JPEG, PNG, WebP or GIF, and one larger than
   4 MiB. Confirm each is refused before anything is sent, with text naming the
   cause.
5. Restart the local server, then reload a page holding a foreign image. Confirm
   the stale handle draws as a missing image rather than an error.
6. Delete an item from the metadata manager and confirm the confirmation names
   the item and demands its name typed. Confirm no other screen deletes an item.

## Collections and playlists across clients

1. Create, rename, add to, remove from and delete a collection; do the same for
   a playlist. Confirm each change is visible in the Jellyfin web client.
2. File one item into a playlist twice, confirm two entries, remove one, and
   confirm the other stands.
3. Reorder a playlist, leave the screen, and confirm a later visit shows the
   order it was left in.
4. Play All on a collection and on a playlist, and confirm each queues in that
   object's own order. Play one item from inside a playlist and confirm the
   remainder follows it.
5. Reorder the queue and confirm it plays in the order it was left. Confirm the
   remote target's queue and a group's queue carry no reorder control.
6. Start a library scan and confirm a burst of changes reaches every open tab as
   one message a second, and that applying it moves neither the scroll position
   nor the sort.
7. Delete an item while its own detail screen is open in another tab, and
   confirm that tab shows text naming the cause rather than stale metadata.
8. Run with `--read-only`: confirm a card's overflow menu offers the play
   control alone, and that every collection, playlist, queue-order and
   metadata-manager control is absent rather than disabled, while hubs,
   filtered lists, filters, sort, the letter jump, trickplay, chapter images,
   Latest rows, the Programs tab and channel detail all work unchanged.

## Playback parity

- `cargo test` in `jellium-web` runs the port and the pinned reference under
  one Node process against one installed environment, and fails on the first
  byte by which a device profile, a browser detection, an hls.js eligibility or
  a secondary-audio answer differs.
- `cargo test -p jellium-cli web::playback::negotiate::differential` runs the
  port and the pinned reference against the same `PlaybackInfo` cases and fails
  on the first byte by which a body differs. It lives beside the serializer it
  measures, since `jellium-cli` has no library target for a test under `tests/`
  to link against.
- `cargo test -p jellium-cli web::playback::requests` counts what each step of
  the playback chain puts on the wire against the stub upstream.
- `just pinned <jellyfin-web-checkout>` rewrites `jellium-web/reference`,
  `reference/spans`, `jellium-web/fonts`, `jellium-web/icons`,
  `jellium-web/branding`, `reference/assets.tsv` and
  `reference/breakpoints.tsv` from the checkout and fails on any difference.
  Its `git ls-files` guard refuses an untracked slice or an untracked span, so
  this command answers only once `jellium-web/reference/jellyfin-web.mjs`,
  `environment.mjs` and `reference/spans` are committed; before that commit it
  reports the tree broken when the tree is not.
- `cargo test -p jellium-reference` digests every row of
  `reference/provenance.tsv` against the span committed under `reference/spans`
  and takes no checkout, so a row whose lines, count or hash have drifted fails
  on a fresh clone.
- A 4K HDR HEVC title on Firefox plays: the network panel shows one
  `POST /Items/{id}/PlaybackInfo` and no unprofiled `GET`.
- Starting a title with twenty-eight subtitle streams issues at most one
  `Stream.vtt` request, and it is the selected track's.
- Switching to a second external subtitle track issues no `PlaybackInfo`.
- Switching from a burned-in track issues two `PlaybackInfo` requests, the
  first carrying `SubtitleStreamIndex: -1`.
- Changing quality issues `DELETE Videos/ActiveEncodings` twice and no
  `Sessions/Playing/Stopped`.
- Every playback request carries an `Authorization` header naming
  `Jellyfin Web`, `10.11.11`, the announced device and the announced device id.
- Playing an item with cinema mode on requests `Intros` and plays its items
  before the item.

## Appearance

What remains here is what a machine cannot see: rasterization, perceived
weight, and whether the screen reads as the same page as the reference's.

Which constructs a screen draws, in which role, carrying which of the
reference's own sentences, is read by
`jellium-reference/tests/constructs.rs` against `reference/constructs.tsv`, and
is not listed here. A row deleted from this section was deleted because that
gate now checks it, not because it stopped mattering.

Each subsection names the viewport sizes it is run at. Run each against a
phone, a tablet, an iPad asking for the desktop site, a desktop browser with a
touch screen and a television browser, and open jellyfin-web's own page beside
it at the same width.

### The shell

Run at 360x800 and 1920x1080.

- The browser tab carries Jellyfin's own favicon.
- The boot screen stands Jellyfin's splash logo, its banner at a device width of
  992 pixels and wider and its icon narrower, and no lettering of the app's name.
- No instant between the boot screen and the canvas's first frame paints the page
  anything but #101010.
- No viewport width produces a horizontal page scrollbar.
- Body text draws in Noto Sans, and every heading draws in that face at its
  regular weight.
- No control's label is upper-cased.
- A submit control stands on #00a4dc and a raised control on #303030.
- A link-style control draws its lettering in #00a4dc on no face of its own,
  and rules that lettering only while the pointer stands over it.
- A title in Cyrillic, Greek, Vietnamese or Devanagari draws in Noto Sans.
- A title in Japanese, Korean or Chinese draws in the Noto Sans family for its
  script, fetched from this origin, and a second title that same face covers
  fetches nothing.
- Every Material Icons glyph draws in a line box its own size tall.
- A phone draws every page at a 90% root, a desktop browser at 93%, and a
  television browser at 125%.
- The layout does not follow the viewport width: a desktop browser draws the
  item detail head beside its poster at 360x800 and at 1920x1080 alike, and a
  phone draws it stacked at 360x800 and at 800x360 alike.
- An iPad reporting `iPad` draws the stacked head; the same iPad asking for the
  desktop site draws the head beside its poster.
- A desktop browser with a touch screen draws the head beside its poster.

### The home screen

Run at 360x800, 800x360, 768x1024 and 1920x1080.

- Opened beside jellyfin-web's own `#/home` at the same width, the two pages
  read as the same page: the same header, the same strip, the same rails in
  the same order at the same rhythm.
- The header's lettering and glyphs sit on the same baseline as the reference's
  and carry the same weight.
- A home rail's cards run to the window edge, and no page-wide gutter stands
  where the reference has none.
- A home rail's card takes its width from the whole window rather than from the
  page inside it: at 1920 wide a next-up card spans 18.7% of the window and a
  latest movie card 10.41%; at 768 wide, 45.5% and 23.1%; at 800 wide, 23.1%
  and 18.5%; at 360 wide, 72% and 40%.
- My Media's tiles take the backdrop rail's own widths, so one spans 18.7% of
  the window at 1920 wide, 45.5% at 768, 23.1% at 800 and 72% at 360.
- At 1920 wide the browser's network panel shows a My Media tile's image and a
  next-up card's image asked for at 355 pixels wide, and a movie library grid's
  poster asked for at 213.
- At 768 wide the network panel shows a My Media tile's image asked for at the
  same width as a next-up card's image, and wider than a movie library grid's
  poster.
- A card whose item carries a BlurHash draws that hash decoded over the whole of
  the card's image box before its own image arrives, edge to edge with no card
  background above or below it, and the image replaces it at the same size once
  it loads; no card carrying a hash shows empty space while its image is in
  flight.
- A loaded card image runs to all four edges of the box its BlurHash filled,
  cropped at its middle where the served image's own shape differs from the
  card's: no card background stands above, below or beside a card's image.
- A library the server holds an image for draws that image on its tile, and the
  browser's network panel shows no image request for a library it holds none
  for.
- The On Now row scrolls sideways at one backdrop rail card's own height, and
  its foot lines up with the Next Up row's below it.
- An On Now card's elapsed bar stands inside its image at the foot rather than
  under the card's name.
- A Continue Watching card draws a bar across the foot of its image at the
  fraction of the item the server holds as played, and a card the server holds
  no position for draws none.
- An On Now card draws that bar at the fraction of its programme's own airing
  that has run.
- A Continue Watching episode card writes its series' name on the first line
  and `S1:E1 - ` before the episode's own title on the second, and a Continue
  Watching movie card writes its name over its year.
- A Next Up card writes its series' name over the episode's own title and no
  year under it, and a music library's Latest rail writes an album's artist
  under its name and no year.
- A card for an episode of season zero writes `Special - ` before the
  episode's own title.
- A programme numbering itself zero and carrying no episode title writes that
  programme's name once rather than on both of its lines.
- A books, home-videos, collections or playlists Latest rail writes each card's
  name over one blank line, and a photos Latest rail writes two blank lines and
  no name; every card in either rail stands the same height as a movies Latest
  card.
- A card's blank line stands at the card's own body size rather than the
  smaller secondary size, so a Latest rail of books is as tall as one of
  movies.
- The On Now rail writes a programme's name, its own title and the times it
  runs, on three lines.
- A television browser's home rails take their own widths at every window
  width: 23.5% of the window for a backdrop rail, 18.8% for a small-backdrop
  rail and 15.6% for a square or portrait rail.
- A television browser asks for a portrait or square card's image at a sixth of
  the page width and a backdrop card's at a quarter, at every window width.

### The library screens

Run at 360x800, 768x1024 and 1920x1080.

- A library the server holds no image for draws its collection's own glyph —
  movie for a movie library, tv for a show library, music_note for a music
  library — over its name, centred on one line.
- A movie the server holds no image for draws the movie glyph at 5em over its
  own background, on a home rail, in a library grid and in a search result
  alike, and a music album the server holds no image for draws the album glyph.
- An episode the server holds no image of its own draws its series' image, a
  track with none draws its album's, and a season with none draws its own
  thumb.
- The browser's network panel shows no image request for an item carrying no
  image tag of any kind.
- A television browser lays four backdrop cards, six square cards and six
  portrait cards across a library grid at every window width, where a desktop
  browser at 383 wide lays one, two and three.
- On a desktop browser a library card under the pointer raises a scrim carrying
  a play disc at its middle and the played mark, the rating control and the more
  control in that order at its trailing foot.
- A phone raises no scrim over any card; it draws the reference's own overlay
  instead, and a television browser draws neither.
- A card whose item is marked played draws its check in #cc3333 and one whose
  item is a favourite draws a filled heart in #cc3333, both drawing in the
  scrim's own lettering otherwise, and pressing either changes the colour
  without a reload.
- A series card the server reports unplayed episodes for draws that count in
  white on a #00a4dc disc at the top trailing corner of its image, `99+` above
  ninety-nine, lettered no heavier than the card's own name.
- A series card whose episodes are all played draws a white check on that disc
  in place of the count, and a card whose item can carry no played mark draws
  neither.
- An album card in a music library grid writes its own name over its album
  artist and writes no year, and a movie card writes its name over its year.
- A search result for an episode, a song or a video writes its parent's name
  over its own and writes no year, where one for a movie, a series or an album
  writes the year.
- A people or artists listing writes one line under a card, where a genres or
  studios listing writes two.
- A continuing series writes `2010 - Present` under its name and one that has
  ended writes `2010 - 2015`, on a shows grid, a Latest rail, a suggestions
  rail and a search result alike.
- A music-video, home-video, book, collections, playlists or untyped library
  grid writes an item's parent title over its name and its year under it, where
  a movies grid writes the name over the year.
- A search result for a programme writes the programme's name, the day and time
  it airs and its channel, on three lines and with no separate title line.
- Under `--read-only` the scrim and the sheet alike offer the play control
  alone, on a library grid, a search result and a genre card.
- The search field stands a 2em magnifier before its 1.1em field, gapped a
  quarter of the magnifier's own em and lifted a tenth of that em off the foot
  of the field's row, the two capped together at a 60em measure centred in the
  page.

### The item detail page

Run at 360x800, 800x360 and 1920x1080.

- A television browser draws item detail with no backdrop, its poster at the
  page's leading edge and the head beside it.
- On a phone the stacked head's poster hangs below the backdrop and over the
  block carrying the item's name and its row of buttons, and that name and that
  row stand to its trailing side rather than beside empty space.
- On a phone the stacked poster's leading edge stands a twentieth of the window
  in at 800 wide and 3.3% of the window in at 360 wide.
- On a phone at 800x360 the stacked poster stands no taller than four fifths of
  the window and is neither squeezed nor cut to the backdrop's height.
- A detail page draws its played mark only where the item can carry one, its
  rating control only where the item can carry one, and a more control wherever
  the item offers a command.
- Opening a collection draws the item detail page: its backdrop, its poster, its
  name, Play All and Shuffle, its overview, and its items beneath.
- Opening a programme from the guide draws the item detail page: its channel by
  name and number above its name, its start and end times, its live, new,
  premiere and repeat flags, its overview and its genres.
- A programme's page offers Play only while the programme is on air, and offers
  Record, Record Series and their cancels only to a user the server lets manage
  Live TV.
- Removing an item from a collection is offered from that item's menu on the
  collection's own page, and from nowhere else.
- A season card under a series writes its name alone.

### Live TV

Run at 768x1024 and 1920x1080.

- A search result raises the library grid's own scrim, and a genre, studio,
  network, artist or album-artist card raises it carrying the more control
  alone.
- That scrim carries no play disc on an item the server holds no file for, no
  played mark on a programme or a channel, and no rating control on a
  programme, a library or a channel.
- A Live TV recording card raises a play control under the pointer, one still
  being written included.
- The latest recordings tab writes a recording's series name over its own title
  and the year it was made, and a recording naming no series writes its name
  over that year.
- Each Programs rail writes the time a programme airs under it: the On Now rail
  writes start and end, the Shows, Sports and Kids rails write the day and the
  start, the Movies rail writes no series name above that day, and the News rail
  writes the programme's name alone above it.

### Playback and the video display

Run at 360x800 and 1920x1080.

- The now-playing bar is 4.2em tall, and its artwork 4.2em wide and 70% of the
  bar's height.
- The video display stands previous, rewind, play, fast forward and next
  against the panel's leading edge, previous and next only while the queue
  holds more than one item, and the ends-at text after them fills the rest of
  the row so subtitles, audio, volume and settings stand at the trailing edge,
  full screen last.
- The video display's control row carries no shuffle, repeat, queue, cast or
  watch-together control, its header stands cast once, and watch-together
  stands once in that header where the server grants SyncPlay and nowhere where
  it does not.
- The video display drops the ends-at text at 75em and narrower, the two seek
  controls at 50em and narrower and the volume control at 43em and narrower, and
  stands its controls shoulder to shoulder at 33.75em and narrower.

### The menus and sheets

Run at 360x800 and 1920x1080.

- A card's overflow menu opens as one sheet on its own rounded surface, with
  every command stacked one under the next and Cancel at the foot.
- The video display's settings, audio, subtitle, quality, repeat and version
  menus each open as one such sheet, titled by the menu's own name, with Cancel
  at the foot.
- The audio, subtitle, quality, repeat and version menus each draw a tick
  against the one entry in force, and every other entry in each of them lines up
  beside that tick.
- The device picker on the Remote screen opens as one sheet titled Play On, each
  target carrying the television glyph, its device name, and the name of the
  client it runs beneath.
- The SyncPlay screen, while this browser is in no group, opens as one sheet
  titled Join a group, each group carrying the person glyph, the group's name
  and its participants beneath, with New group under the plus glyph last where
  the server lets this user create one.
- The SyncPlay screen, while this browser is in a group, opens as one sheet
  titled with the group's own name over its participants, offering Stop local
  playback under the pause glyph and Leave group under the door glyph, each with
  its own sentence beneath.
- A sheet whose rows carry no glyph and which carries no title stands its rows
  against the leading edge with no room reserved before them.

### The settings region and the dashboard

Run at 768x1024 and 1920x1080.

- On the logs screen and on the general settings, transcoding and trickplay
  screens, a checkbox draws one glyph: an outlined box while it is cleared and a
  box filled in #00a4dc while it is set, with a disc behind it under the pointer.
- On the Networking screen and on the user's own Display, Home and Playback
  settings screens, a checkbox draws an outlined box edged in the page's own
  lettering while it is cleared and filled with #00a4dc while it is set.
- The activity log draws its user column in the All and User views and drops it
  in System.

### The signed-out pages and the setup wizard

Run at 360x800 and 1920x1080.

- The select-server page and the login page each stand Jellyfin's own banner in a
  header slot 13.2em wide.
- The add-server page, the password-reset page and every setup wizard step each
  stand that same banner in that same slot.
- The select-server page centres its cards, each a square card carrying the
  storage glyph over a centred name, and a name wider than its card ends in an
  ellipsis on one line.
- The login page's user cards and its sign-in form each stand centred in a page
  padded 3.30% at each side.
- A user card's name stands the same distance off the top of its footer as its
  last-seen line stands off the name, and an account with no name writes an
  empty first line rather than a blank one.
