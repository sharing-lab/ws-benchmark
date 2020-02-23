#!/usr/bin/perl -wl
use strict;

for (0 .. 6) {
	my $c = 2**$_;
	my $cmd = "wrk -t1 -c$c -d10s http://127.0.0.1:$ARGV[0]/color";
	print "\n", $cmd;
	system $cmd;
} 
