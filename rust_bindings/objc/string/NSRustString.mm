#import <CoreFoundation/CoreFoundation.h>
#import <Foundation/Foundation.h>
#include "NSRustString.h"

@implementation NSRustString

+ (instancetype)withRustString:(::rust::String)str {
    return [[self alloc] initWithRustString: str];
}

- (id)init {
    self = [super init];
    return self;
}

- (id)initWithRustString:(::rust::String)str {
    if(self = [super init]) {
        _backingStr = str;
        _cfStr = (__bridge_transfer NSString*) CFStringCreateWithBytesNoCopy(
            kCFAllocatorDefault,
            reinterpret_cast<const UInt8*>(_backingStr.data()),
            _backingStr.length(),
            kCFStringEncodingUTF8,
            false,
            kCFAllocatorNull // Do not deallocate the buffer
        );
    } return self;
}

- (NSUInteger)length {
    return _cfStr.length;
}

- (unichar)characterAtIndex:(NSUInteger)index {
    return [_cfStr characterAtIndex:index];
}

- (void)getCharacters:(unichar *)buffer range:(NSRange)range {
    return [_cfStr getCharacters:buffer range:range];
}

- (NSStringEncoding)fastestEncoding {
    return [_cfStr fastestEncoding];
}

- (NSStringEncoding)smallestEncoding {
    return [_cfStr smallestEncoding];
}

- (NSUInteger)cStringLength {
    return [_cfStr cStringLength];  
}

- (void)getLineStart:(NSUInteger *)startPtr
                 end:(NSUInteger *)lineEndPtr
         contentsEnd:(NSUInteger *)contentsEndPtr
            forRange:(NSRange)range {
    return [_cfStr getLineStart:startPtr
                            end:lineEndPtr
                    contentsEnd:contentsEndPtr
                       forRange:range];
}

- (const char*)UTF8String {
    return [_cfStr UTF8String];
}

- (BOOL)isEqualToString:(NSString *)aString {
    return [_cfStr isEqualToString:aString];
}

- (BOOL)isEqual:(id)aString {
    return [_cfStr isEqual:aString];
}

@end

/*
-(unsigned long long)hash;
-(BOOL)hasPrefix:(id)arg1 ;
-(BOOL)hasSuffix:(id)arg1 ;
-(id)substringWithRange:(NSRange)arg1 ;
-(id)copyWithZone:(NSZone*)arg1 ;
-(Class)classForCoder;
-(unsigned short)characterAtIndex:(unsigned long long)arg1 ;
-(BOOL)getCString:(char*)arg1 maxLength:(unsigned long long)arg2 encoding:(unsigned long long)arg3 ;
-(id)mutableCopyWithZone:(NSZone*)arg1 ;
-(const char*)cStringUsingEncoding:(unsigned long long)arg1 ;
*/
