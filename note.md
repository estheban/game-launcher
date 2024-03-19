

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
