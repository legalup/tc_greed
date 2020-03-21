# -*- coding: utf-8 -*-
"""
Created on Thu May 11 19:44:52 2017

@author: luis
"""

import subprocess
import sys


if __name__ == "__main__":
    
    proc = subprocess.Popen(["/home/luis/workspace/tc_greed/cf/cf"], shell=False, stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    fh = open(sys.argv[1], "r") 
    inputstr=""
    for line in fh: 
        words = line.split()
        for wrd in words:
            inputstr += wrd + "\n"
    output = proc.communicate(input=inputstr)[0]
    print output
    
    
