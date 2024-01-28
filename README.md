# kitchen

It's a remote CTF environment on AWS (someone add more cloud providers) with immediate region migration. Created for pwners with Apple silicon and need to travel to in-person CTFs (rip)

## usage

- kitchen: ssh into box in current ctf folder
- kitchen config: set config shit
- kitchen migrate [region]: ping all AWS regions and migrate to closest one (or one of your choosing)
- kitchen workon [ctf]: get or set current ctf.
- kitchen share [dir]: sync directory (current directory is default) with remote one

## config

idk

## recommended software

docker, tailscale
