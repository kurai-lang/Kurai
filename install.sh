USER_ID=$(id -u)
is_root=false

if [ "$USER_ID" -eq 0 ]; then
    is_root=true
fi

if [ "$is_root" = true ]; then
    sudo ln -s "$(realpath target/release/vync)" "/usr/local/bin/vync"
    echo -e "\033[1mVync successfully installed to system. Located at \`/usr/local/bin/vync\`\033[0m!"
else
    echo -e "\033[1mYou are NOT root!\033[0m\nplease run this with \`sudo\`!"
fi
