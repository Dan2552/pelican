#include <stdio.h>
#import <Foundation/Foundation.h>

void objc_enable_momentum_scroll() {
    [[NSUserDefaults standardUserDefaults] setBool:YES forKey: @"AppleMomentumScrollSupported"];
}
