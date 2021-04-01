#ifndef NSRustString_h
#define NSRustString_h

#include "rust/twitter-text/twitter-text.h"
#import <Foundation/Foundation.h>

@interface NSRustString : NSString {

#ifdef __cplusplus
    ::rust::String _backingStr;
    NSString* _cfStr;
#endif

}

#ifdef __cplusplus

+(NSRustString*) withRustString:(::rust::String)param;

#endif

@end

#endif /* NSRustString_h */
