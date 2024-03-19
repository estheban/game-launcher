#!/bin/bash

export AWS_PROFILE=elcweb
export TAURI_PRIVATE_KEY=~/.tauri/myapp.key
export TAURI_KEY_PASSWORD=

npm run tauri build

aws s3 cp src-tauri/target/release/bundle/macos/game-launcher.app.tar.gz s3://yulbrew-game-launcher-dev/

export FILE_SIGNATURE=`cat src-tauri/target/release/bundle/macos/game-launcher.app.tar.gz.sig`
export OLD_SIGNATURE="dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkKUlVTaWN4OFp3dHNoS1FEZC84U3orOC8wNUlIMkhEOVVUY1k2cVMxL2RVbWJOYmJaT0ZvTlAzQitoVUZ5cGh0WUhhOUhUNDJmQnZIVmxvWXRHQ1ljbG13cTBjRlc2TkNyemdrPQp0cnVzdGVkIGNvbW1lbnQ6IHRpbWVzdGFtcDoxNzEwODU3NzIxCWZpbGU6Z2FtZS1sYXVuY2hlci5hcHAudGFyLmd6CjJoYTA0VmFtZDdlclRsLzYzNElaQzZQVXBlTnVQbHVsNlRBaUEyamN2ZGg2QXBQNlJaSXJ6ajdndUJFWmFkNVlQQW5tazMrMGJZNVNWd09XQjZudkRBPT0K"

if [ "$FILE_SIGNATURE" = "$OLD_SIGNATURE" ]; then
    echo "Signature is the same."
else
    echo "Signature changed."
    echo $FILE_SIGNATURE
fi
