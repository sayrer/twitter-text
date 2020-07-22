import os
import sys
import pprint
import setuptools
import modtest
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

if __name__ == "__main__":
    # execute only if run as a script
    main()