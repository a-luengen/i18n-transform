- [1. Introduction](#1-introduction)
- [2. How-to use](#2-how-to-use)

# 1. Introduction

A little cli tool written in rust to transform directories containing translation files 
from and to csv files, to simplify translation management.

# 2. How-to use

## 1. Create from react-native-localization conform directory:

itransf --to <directory_path> --output <target_filepath_csv>

## 2. Create from reactjs next-i18next conform directory

itransf --to <file_path> --output <target_filepath_csv>

## 3. Transform into react-native-localization conform directory

itransf --from <src_filepath> --output <target_directory_path>

## 4. Transform into reactjs next-i18next conform directory

itransf --from <src_filepath_csv> --output <target_directory_path>

