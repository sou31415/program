#include <bits/stdc++.h>
using namespace std;
#define rep(i,n) for(int i = 0;i < (n);i++)
int main(){
	string s = "atcoder";
	int a,b;
	cin >> a >> b;
	--a;
	for(int i = a;i<b;i++){
		cout << s[i];
	}
	cout << endl;
	return 0;
}
