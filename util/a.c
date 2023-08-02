#import <stdio.h>
#import <stdlib.h>
#define rep(i,s,n) for(int i = (s);i < (n);i++)
#define isMin(a,b) a < b? 1:0
#define isMax(a,b) a > b? 1:0
void SimpleSort(int,int*);
void SelectionSort(int,int*);
void InsertionSort(int,int*);
void GenerationDataSet(int,int,int*);
void output(int,int*);
double Random();
int main(){
	int *pointer;
	int n,s;
	int max;
	long long seed;
	printf("データ数:");
	scanf("%d",&n);
	printf("最大値:");
	scanf("%d",&max);
	pointer = (int *)malloc(sizeof(int)*n);
	GenerationDataSet(n,max,pointer);
	printf("Sort Algorithmを選択してください\n1:最も単純 2:選択法 3:挿入法\n");
	scanf("%d",&s);
	if(s == 1){
		SimpleSort(n,pointer);
	}else if (s == 2){
		SelectionSort(n,pointer);
	}else{
		InsertionSort(n,pointer);
	}
	output(n,pointer);
	return 0;
}
void InsertionSort(int n,int *a) {
	int i,j,w;
	rep(i,1,n) {
		w = a[i];
		j = i - 1;
		while ((j >= 0) && (w < a[j])) {
			a[j+1] = a[j];
			--j;
		}
		a[j+1] = w;
	}
}
void swap(int *a,int *b) {
	int d;
	d = *a;
	*a = *b;
	*b = d;
}
void SelectionSort(int n,int *pointer) {
	int mpos;
	rep(i,0,n-1) {
		mpos = i;
		rep(j,i+1,n) {
			if (pointer[mpos] > pointer[j]) {
				mpos = j;
			}
		}
		swap(&pointer[mpos],&pointer[i]);
	}
}
void SimpleSort(int n,int *pointer) {
	rep(i,0,n-1) {
		rep(j,i+1,n){
			if (pointer[i] > pointer[j]) {
				swap(pointer+i,pointer+j);
			}
		}
	}
}
void GenerationDataSet(int n,int max,int *data) {
	rep(i,0,n) {
		data[i] = (int)(max*Random() + 1);
	}
}
void output(int n,int *data){
	rep(i,0,n) {
		printf("%d ",data[i]);
	}
	printf("\n");
}
double Random(){
	double x = (double)rand()/(double)(RAND_MAX + 1.0);
	return x;
}
