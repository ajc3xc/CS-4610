It's time for a key-signing party!

![](https://imgs.xkcd.com/comics/responsible_behavior.png)

https://xkcd.com/364/

# Programming assignment 5 (pa05) -- Practical, usable, end-to-end user cryptography
This is a different kind of assignment! 
There are 5 sub-parts:

## Part 1: ssh key
Create an ssh key, 
set it up to log into gitlab/git-classes,
and take screenshots of it functioning.
To learn how, first read the following links in full:
* https://git-classes.mst.edu/help/user/ssh
* https://help.github.com/articles/connecting-to-github-with-ssh/ (and sub-pages)
* https://www.digitalocean.com/community/tutorials/understanding-the-ssh-encryption-and-connection-process

What to submit:
* Deliverable 1: `ssh-key0.png` which should contain an image of the browser interface with your public key in it.
* Deliverable 2: `ssh-key1.png` which should contain an image of terminal where you did an operation like git pull, with no need for a remote username/password. 
* Deliverable 3: `id_rsa.pub` which is your RSA public key from your ~/.ssh/ folder (note this is NOT the secret key; don't give me that).

Note: 
If you use your public key for real things, and care to preserve privacy,
then you don't have to give me your real public key,
if you don't care to (though cryptographically it should not matter);
it just has to be a valid one with that name.

## Part 2: gpg2 email
Set up email encryption using gpg2.

a. Read the content at all of the crypto-related links here: 
   https://cs-mst.gitlab.io/index/ContactPublicKey.html

b. Read about how to use gpg2 at the command line in Linux/Unix
   https://duckduckgo.com/?q=gpg2+command+line

c. Create a gpg2 key with 4096 bit RSA and your campus email,
   and put your email in the `email_used_for_gpg2_key.txt` file in the repo.

Hint: often the command line interface, `gpg2` is easier,
and it might be smoother to use it here,
especially for creating the key,
but you can try various GUIs for using it if you want.

```sh
# actually read a bit:
man gpg2
```

d. Set up your email a in mail client (optional).
   You may consider the following for supported clients: 
   https://en.wikipedia.org/wiki/GNU_Privacy_Guard#Application_support
   (Evolution, KMail, Thunderbird, Claws-mail are nice, or Mutt if you dare try the Vim of email clients) 
   or use webmail to keep it easy with https://www.mailvelope.com/en/
   If you go with mailvelope, 
   then you may need to generate your keys manually to get the right parameters.

Warning: 
Don't use accidentally use pop3 to download and delete all your email from the server 
(unless you actually want to)!

e. Exchange keys with 5 students in this class. 
   Do this either in person, or using a keyserver like https://keys.mailvelope.com/ with any client you choose.
   You need to actually read the gpg2 manual or gpg2 command line tutorials,
   to make sure you know how to export and import keys.

f. Get those same 5 students to sign your gpg2 public key at the command line in Linux, using gpg2;
   Include your 5x signed key in your repo, and optionally (above) on the mailvelope or other keyserver.
   Note: **actually** spend time to read about how to work gpg2 at the command line in Linux.
   As an overview:
   generate your key, export it, send it to a friend, 
   have them import, sign it, export, and send back to you, 
   where you import, and re-export, 
   and then do the same thing 4 more times, 
   serially, so you actually get the signatures in one final export. 

g. Encrypt separate back-and-forth discussions with 5 different students in this class.
   Take a single screenshot for each back-and-forth exchange.

What to submit:
* Deliverable 1: `gpg2_conversation1.png`, `gpg2_conversation2.png`, `gpg2_conversation3.png`, `gpg2_conversation4.png`, `gpg2_conversation5.png`,  where the png files show evidence of your encrypted signed conversations.
* Deliverable 2: `gpg2_key.asc` the ascii-armored exported gpg2 key with five other student signatures embedded in it.

## Part 3: Download verification
Fully validate a `whateverLinux.iso` you download.
Take screenshots of two parts: 
1. ISO hash checking and 
2. verification of gpg2 signature on hash (which builds on skills from the previous task).

To learn about this process, read some examples from OpenSuse, Debian, Kali, Ubuntu:
* https://en.opensuse.org/SDB:Download_help#Checksums
* https://www.debian.org/CD/faq/index.en.html#verify
* https://www.debian.org/CD/verify
* https://docs.kali.org/introduction/download-official-kali-linux-images
* https://www.kali.org/downloads/
* https://tutorials.ubuntu.com/tutorial/tutorial-how-to-verify-ubuntu#0

What to submit:
* Deliverable 1: `iso-verify0.png` where you show the commands and results to verify your generated hash of the iso against the downloaded/reported hash.
* Deliverable 2: `iso-verify1.png` where you show the commands and results to verify the verify the signature on the hash using the dev pgp key you download.

Hint: 
Fedora, Debian, OPenSuse, and various other distributions post their developer signatures and hashes,
in more-or-less accessible locations, on their websites (you may need to search around to find them).

## Part 4: FS secure chat
With 5 students in the class (can be different or the same as for previous part), 
exchange back-and-forth communications in an open-source end-to-end forward-secure app of your choosing.
    
1. Read: 
* https://en.wikipedia.org/wiki/Forward_secrecy
* https://en.wikipedia.org/wiki/Deniable_authentication
* https://cs-mst.gitlab.io/index/Classes/Security/Content/12b-DeniableForwardSecure.html

2. Then choose an app:
* https://briarproject.org/ (Android, fully p2p, even does local meshnet for when cell goes down)
* https://ricochet.im/ (fully p2p, Linux/Mac/Windows)
* https://retroshare.cc/ (fully p2p, very flexible, does video and file sharing, Linux/Mac/Windows)
* https://tox.chat/ (cool tech, fully p2p, all platforms including mobile)
* https://jami.net (cool tech, fully p2p)
* https://wire.com/ (easy/friendly, but centralized, all platforms including mobile)
* https://www.signal.org/ (ok, centralized)
* https://silence.im/ (Android and sms only, but distributed)
* OTR (many clients on Jabber/XMPP, IRC, etc) or OMEMO. 
  Both are open federated protocol with multiple applications
    * https://en.wikipedia.org/wiki/Off-the-Record_Messaging
    * https://en.wikipedia.org/wiki/OMEMO
    * https://conversations.im/ (Android)
    * https://gajim.org/ (Linux)
    * https://chatsecure.org/ (ios)
* http://goldbug.sourceforge.net/ (not really sure what's up with this one)

These are the main contenders when it comes to open-source perfect-forward-secure text-based communications.
If you are aware of any others, 
please feel free to check with me to see if they meet the criteria,
and I can add them to the list.

3. Include screenshots in the repository of 5 exchanges.

What to submit:
* Deliverable: `pfs_conversation1.png`, `pfs_conversation2.png`, `pfs_conversation3.png`, `pfs_conversation4.png`, `pfs_conversation5.png` which show evidence of 5 class conversations.

## Part 5: Secure git repos
Check out my signature on the latest git commit!
`git log --all --graph --show-signature`

Use your new gpg2 key to sign your git commit for this assignment.
`man git commit` or `git commit --help`

Notice that when you type the following,
that your signatures on the commits show up!
`git log --all --graph --show-signature`

Deliverable: `git_secure.png` should show you typing `git log` and your signed commit appearing in the commit history.

## Git autograding
Git clone your repository onto your machine, and run the autograder.
See: [docs/git_autograding.md](docs/git_autograding.md)
