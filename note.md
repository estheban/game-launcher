

Game build
- list of files with md5 checksum

Todos:
- GitHub updates
  - [ ] Launcher CI 
  - [ ] On Latest release
    - update default exe/dmg files
    - enable previous version to download that one
- [ ] Setup download folder
- [ ] Get game version
- [ ] Download game build definition
- [ ] Download / update button
- [ ] Download process
  - get game build definition
  - for each file in game build definition
    - if file exists
      - check md5
      - if md5 is wrong, download again 
    - download file
    - check md5
    - if md5 is wrong, download again
- [ ] Settings menu and option to change download folder



# Data model

App: represent one app or one game or a DLC
  - uuid
  - name
  - type
  - developer
  - publisher
  - franchise
  - supported systems
  - technologies
  - release date
  - description
  - image
  - last update date
  - app_directory
  - status (released, unreleased, ...)
  - images
    - icon
    - logo
    - header
    - 
Depots: represent a collection of file for a purpose, ex: the windows version of the game or macos version of the game



share price
- Steam: 30%
- Epic: 12%
  - no royalties on unreal engine games sold on the store
  - creator revenue share (affiliate link) from the 12% cut of Epic
// the rest is ai fetch, not validated
- GOG: 30%
- Itch.io: 10%
- Humble: 15%
- Microsoft: 15%
- Nintendo: 30%
- Sony: 30%
- Apple: 30%
- Google: 30%
- Amazon: 30%
- Facebook: 30%
- Discord: 10%
- Twitch: 25%
- Origin: 30%
- Uplay: 30%
- Battle.net: 30%
- Bethesda: 30%
- Rockstar: 30%
- Square Enix: 30%
- Ubisoft: 30%
- EA: 30%
- Activision: 30%
- 2K: 30%
- Capcom: 30%
- Konami: 30%
- Sega: 30%
- Bandai Namco: 30%
- CD Projekt: 30%
- Valve: 30%
- 