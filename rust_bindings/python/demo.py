import os
import sys
import pprint
import setuptools
import twitter_text
 
def main():
    print("hi")
    import sys
    import bz2
    print("Python version")
    print (sys.version)
    print("Version info.")
    print (sys.version_info)
    autolinker = twitter_text.Autolinker()
    print(autolinker)
    print(autolinker.autolink("yo $TWTR yo"))
    config = twitter_text.TwitterTextConfiguration()
    print(config)
    print(dir(config))
    print(config.get_version())

if __name__ == "__main__":
    # execute only if run as a script
    main()