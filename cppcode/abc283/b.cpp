#include <bits/stdc++.h>
using namespace std;

int main(){
	int q , n , c , k , x;
	cin >> n;
	vector<int> a(n);
	for (int i = 0;i < n;i++) cin >> a[i];
	cin >> q;
	for(int i = 0;i < q;i++){
		cin >> c >> k;
		if (c == 1){
		   cin >> x;
		   a[k-1] = x;
		}else{
			cout << a[k-1] << "\n";
		}
	}
	return 0;
}
