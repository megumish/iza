# Iza
iikanzi ni deploy command

# Install
requirement: Rust v1.37.0

```
git clone git@github.com:megumish/iza.git
cd iza
cargo install --path . --features=exec
```

# How to use

## Initialize Directory

```
iza init
```

## New Credential

```
iza credential ssh new <USER> <HOST>
```

example input 
```
iza credential ssh new user xxx.xxx.xxx.xxx
```
## Credential List

```
iza credential ssh
```

example output 
```
SSHConnection List
id: 010ee301-8054-47ab-8ebe-f1509939e87e
user: user
host: xxx.xxx.xxx.xxx
```

## New Managed File

```
iza object new <LOCAL_PATH> <REMOTE_PATH> <CREDENTIAL_ID>
```

example input
```
iza object new file /home/iza/file 010ee301-8054-47ab-8ebe-f1509939e87e
```

## Iza Deploy

```
iza deploy
```
