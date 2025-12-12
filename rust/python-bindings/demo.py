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
    extractor = twitter_text.Extractor()
    entities = extractor.extract_mentioned_screennames_with_indices("fooo @jack @biz fooo")
    print(entities)
    print(dir(entities[0]))
    print(entities[0].value)

if __name__ == "__main__":
    # execute only if run as a script
    main()