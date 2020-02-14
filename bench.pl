#!/usr/bin/perl -wl
use strict;

for (0 .. 6) {
	my $c = 2**$_;
	my $cmd = "ab -n 4096 -c $c http://192.168.1.12:$ARGV[0]/color";
	print $cmd;
	system $cmd;
} 
