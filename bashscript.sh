#!/usr/bin/env bash
clear

RED='\e[31m'
GREEN='\e[32m'
YELLOW='\e[33m'
NC='\e[0m'
echo -e "${GREEN}Welcome to filestorage bash ui!${NC}"
echo -e "${YELLOW}Please select your action here${NC}"
echo -e "\n\n"
echo -e "${RED}1>${NC} Get information about the files in the server"
echo -e "${RED}2>${NC} Download a file from the server"
echo -e "${RED}3>${NC} Upload a file to the server"
echo -e "\n\n"
read -rp "You chose: " action </dev/tty
echo -e "\n\n"
if [[ $action == "1" ]]; then
  curl 127.0.0.1:8000/bash/get
elif [[ $action == "2" ]]; then
  curl 127.0.0.1:8000/bash/get
  echo -e "${GREEN}Choose an index of file to download${NC}"
  read -rp "Index: " index </dev/tty
  echo -e "${GREEN}Choose a path to save to (including the file)${NC}"
  read -rp "Path: " path </dev/tty
  curl 127.0.0.1:8000/bash/download/"$index" > "$path"
elif [[ $action == "3" ]]; then
  echo -e "${GREEN}Select the file path${NC}"
  read -rp "Path: " path </dev/tty
  echo -e "${GREEN}Enter a file description${NC}"
  read -rp "Description: " description </dev/tty
  curl -X POST -F "file=@${path}" -F "description=${description}" 127.0.0.1:8000/api/upload
else
  echo "This option is invalid"
fi