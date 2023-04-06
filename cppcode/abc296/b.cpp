#include <bits/stdc++.h>
using namespace std;

int main(){
	vector<string> c(8);
	for(int i = 0;i < 8;i++){
		cin >> c[i];
	}
	for (int i = 0;i < 8;i++){
		for(int j = 0;j < 8;j++){
			if (c[i][j] == '*'){
				cout << (char)((int)'a'+ j) << 8-i << endl;
				return 0;
			}
		}
	}
}

