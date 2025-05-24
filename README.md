# Pidarr

**Please do not expose this service to the public, this is a hobby project and my
first code written in Rust, it isn't secure and may give unintended access to
your system, it is designed to be used locally, or from behind a VPN.**

This is a program that fits a very specific need for individuals running a
Jellyfin/*arr stack on very low end hardware (like a Raspberry Pi -- hence the
name). It is a web UI controller and daemon that will monitor your*arr stack in
order to download and transcode media in a space efficient manner. It is for
people who

1. Want all of their media transcoded into a common format (for me this is x264 and a
   bitrate of ~2-3Mbps).
2. Only have access to low-end graphics hardware without the ability to live
   transcode media on demand.
3. Want immediate availability of their media - on download - regardless of
   format.
4. Want to seed all their media to some ratio.
5. (Optionally) want their media to be removed after watched.

It works by having Jellyfin track a "virtual" media library that is composed
entirely of softlinks to the media that is organised by Radarr, and then
transcoded by Tdarr.

The typical lifecycle of a piece of media in the Pidarr system is:

1. New media is added to Radarr. A search begins and a torrent is added to
   qBittorrent.
2. The media finishes downloading and is organised into Radarr's library.
3. Pidarr recognises this media and softlinks it to Jellyfin's library. Your
   meida is now available to stream.
4. Tdarr also recognises this media and begins transcoding it into the
   common format. This is done in the background and does not block the
   availability of the media.
5. When Tdarr finishes transcoding the media, Pidarr recognises this and
   softlinks the transcoded version to Jellyfin's library.
6. Pidarr watches the torrent until it reaches a certain seeding ratio, and
   providing it has already been transcoded, it will remove the torrent and
   associated files, as well as deleting the media from Radarr.
7. (Optionally) once the media has been watched, Pidarr will remove the media
   from Tdarr's library as well, completely removing it from the system - if it
   has not been favourite-ed in Jellyfin.

N.B. This system should be avoided at all costs. It is much better to simply buy
hardware that can handle live transcoding. But if you are a hobbyist, or a poor
student/traveller like myself without extra money for graphics hardware, this is
a hacky solution that works well enough.
