#include "stdc++.h"

using namespace std;

const int INF = 1e4 + 100;
bool True = true;
bool False = False;

struct ORDER {
	int from = 1;
	int to = 1;
	vector<int> way = {};
	int time_start = 0;
	int time_end = 0;
	ORDER(int from_, int to_, vector<int> way_, int time_start_, int time_end_){
		from = from_;
		to = to_;
		way = way_;
		time_start = time_start_;
		time_end = time_end_;
	}
};



struct CAR
{
	int ziel = 0;
	/*
	0 - north
	1 = ost
	2 = west
	3 = south
	*/
	int num = 0;
	int pos = 1;
	bool free = true;
	int end = 0;
  deque <ORDER> order;
	int free_time = 0;
	CAR(int n, int p, int ziel){
		pos = p;
		num = n;
		end = pos;
		ziel = ziel;
	}
  void pop(){
    if(order.size() > 0)order.pop_front();
    if(order.size() == 0)free=true;
  }
  ORDER front(){
    if(order.size() > 0)return order.front();
    return ORDER(0, 0, {}, 0, 0);
  }
  void push(ORDER r){
    order.push_back(r);
    free = false;
		end = *(r.way.end() - 1);
		free_time += r.way.size();
  }
  bool empty(){
    return order.size() == 0;
  }
	int next(int time){
		int it = 0;
		auto s1 = begin(it).way;
		int beta = time - (begin(it)).time_start;
		/*if(order.front().to == s1[beta]){

		}*/
		return s1[beta];
	}

	ORDER begin(int plus){
		//cout<<" ( in PLUS == "<<plus<<" ";
		//cout<<" -> "<<order.size()<<" "<<(*(order.begin() + plus)).from<<(*(order.begin() + plus)).to<<" <- )";
		if(order.begin() + plus != order.end())return *(order.begin() + plus);
		return order[0];
	}
	int size(){
		return order.size();
	}

};

struct MAP
{
		deque <vector<int>> order_list = {};

		CAR eva01 = CAR(1, 36, 1);


		CAR eva02 = CAR(2, 43, 2);


		CAR eva00 = CAR(3, 37, 3);


		void cars(){
			eva01.num = 1;
			eva01.pos = 36;
			eva01.ziel = 3;
			eva01.free = true;

			eva02.num = 2;
			eva02.pos = 43;
			eva02.ziel = 2;
			eva02.free = true;

			eva00.num = 3;
			eva00.pos = 37;
			eva00.ziel = 1;
			eva00.free = true;
		}


		int tick = 0;


		vector<vector<int>> Path = {
			vector <int> {},
			vector <int> { 9, 2 },
			vector <int> { 1, 3 },
			vector <int> { 2, 11, 4 },
			vector <int> { 3, 12, 5 },
			vector <int> { 4, 6 },
			vector <int> { 5, 13, 7 },
			vector <int> { 6, 14, 8 },
			vector <int> { 7, 15 },
			vector <int> { 1, 16 },
			vector <int> { 11 },
			vector <int> { 3, 10, 19},
			vector <int> { 4 },
			vector <int> { 6, 21 },
			vector <int> { 7, 22 },
			vector <int> { 8, 23 },
			vector <int> { 9, 17, 24 },
			vector <int> { 16, 18 },
			vector <int> { 17, 19 },
			vector <int> { 18, 26, 20, 11 },
			vector <int> { 19 },
			vector <int> { 13, 28 },
			vector <int> { 14, 29 },
			vector <int> { 15, 31 },
			vector <int> { 16, 32 },
			vector <int> { 26 },
			vector <int> { 19, 25, 27, 35 },
			vector <int> { 26, 28 },
			vector <int> { 21, 27 },
			vector <int> { 22, 38 },
			vector <int> { 31 },
			vector <int> { 30, 23, 40 },
			vector <int> { 24, 41, 33 },
			vector <int> { 32, 34 },
			vector <int> { 33, 35 },
			vector <int> { 34, 26, 36, 42 },
			vector <int> { 35 },
			vector <int> { 38 },
			vector <int> { 37, 29, 39, 44 },
			vector <int> { 38, 40 },
			vector <int> { 31, 46 },
			vector <int> { 32, 47 },
			vector <int> { 35, 49 },
			vector <int> { 51 },
			vector <int> { 38, 53},
			vector <int> { 46 },
			vector <int> { 45, 54, 40 },
			vector <int> { 41, 48 },
			vector <int> { 47, 49 },
			vector <int> { 42, 48, 50 },
			vector <int> { 49, 51 },
			vector <int> { 50, 52, 43 },
			vector <int> { 51, 53 },
			vector <int> { 52, 44, 54 },
			vector <int> { 46, 53 },
			vector <int> {}
		};

	void cleaner(){
		if((eva00.front()).time_end == tick)eva00.pop();
		if((eva01.front()).time_end == tick)eva01.pop();
		if((eva02.front()).time_end == tick)eva02.pop();
		if(!order_list.empty())order_list.pop_front();
	};


	bool can_use(int to, int time, int num){
		int it = 1;
		if(num == 1){
			//test eva02 and eva00
			if(!eva00.empty()){
				while(eva00.order.begin()+it != eva00.order.end()){

						if(eva00.begin(it).time_end > time - (eva00.begin(it)).time_start){
							auto s1 = eva00.begin(it).way;

							int point1 = time - (eva00.begin(it)).time_start;

							//cout<<endl<<"eva00.free_ time << "<<eva00.free_time<<endl;
							/*cout<<endl<<"way "<<s1.size()<<endl;
							cout<<endl<<"it == "<<it<<" but "<<(eva00.begin(it)).time_start<<" time is "<<time<<endl;
							cout<<endl<<"point1 == "<<point1<<" but "<<s1.size()<<endl;*/

							for(int i = 0;i<Path[s1[point1]].size();i++){
								if(to == Path[s1[point1]][i])return false;
							}
						}
					it++;
				};
			}
			it = 1;
			if(!eva02.empty()){
				while(eva02.order.begin()+it != eva02.order.end()){

						if(eva02.begin(it).time_end > time - (eva02.begin(it)).time_start){
							auto s1 = eva02.begin(it).way;

							int point1 = time - (eva02.begin(it)).time_start;

							//cout<<endl<<"eva00.free_ time << "<<eva00.free_time<<endl;
							/*cout<<endl<<"way "<<s1.size()<<endl;
							cout<<endl<<"it == "<<it<<" but "<<(eva00.begin(it)).time_start<<" time is "<<time<<endl;
							cout<<endl<<"point1 == "<<point1<<" but "<<s1.size()<<endl;*/

							for(int i = 0;i<Path[s1[point1]].size();i++){
								if(to == Path[s1[point1]][i])return false;
							}
						}
					it++;
				};
			}

		}
		if(num == 2){
			it = 1;
			//cout<<"Un";
			if(!eva00.empty()){
				while(eva00.order.begin()+it != eva00.order.end()){

						if(eva00.begin(it).time_end > time - (eva00.begin(it)).time_start){
							//cout<<"it == "<<it<<" when "<<eva00.begin(it).way[0]<<endl;
							vector<int> s1 = (eva00.begin(it)).way;

							int point1 = time - (eva00.begin(it)).time_start;

							//cout<<endl<<"eva00.free_ time << "<<eva00.free_time<<endl;
							/*cout<<endl<<"way "<<s1.size()<<endl;*/
							//cout<<endl<<"it == "<<it<<" but "<<(eva00.begin(it)).time_start;<<" time is "<<time<<endl;
							/*cout<<endl<<"point1 == "<<point1<<" but "<<s1.size()<<endl;*/
							//cout<<"--"<<s1[point1]<<"--";
							for(int i = 0;i<Path[s1[point1]].size();i++){
								if(to == Path[s1[point1]][i])return false;
							}
						}
					it++;
				};
			}

			it = 1;
			//cout<<endl<<"Dead"<<endl;
			if(!eva01.empty()){
				while(eva01.order.begin()+it != eva01.order.end()){

						if(eva01.begin(it).time_end > time - (eva01.begin(it)).time_start){
							//cout<<"it == "<<it<<" when "<<eva01.begin(it).way[0]<<" my size is"<<eva01.order.size()<<endl;
							vector<int> s1 = (eva01.begin(it)).way;

							int point1 = time - (eva01.begin(it)).time_start;

							//cout<<endl<<"eva00.free_ time << "<<eva00.free_time<<endl;
							/*cout<<endl<<"way "<<s1.size()<<endl;*/
							//cout<<endl<<"it == "<<it<<" but "<<(eva00.begin(it)).time_start;<<" time is "<<time<<endl;
							/*cout<<endl<<"point1 == "<<point1<<" but "<<s1.size()<<endl;*/
							//cout<<"--"<<s1[point1]<<"--";
							for(int i = 0;i<Path[s1[point1]].size();i++){
								if(to == Path[s1[point1]][i])return false;
							}
						}
					it++;
				};
			}
		}
		if(num == 3){
			it = 1;
			if(!eva01.empty()){
				while(eva01.order.begin()+it != eva01.order.end()){

						if(eva01.begin(it).time_end > time - (eva01.begin(it)).time_start){
							//cout<<"it == "<<it<<" when "<<eva01.begin(it).way[0]<<endl;
							vector<int> s1 = (eva01.begin(it)).way;

							int point1 = time - (eva01.begin(it)).time_start;

							//cout<<endl<<"eva00.free_ time << "<<eva00.free_time<<endl;
							/*cout<<endl<<"way "<<s1.size()<<endl;*/
							//cout<<endl<<"it == "<<it<<" but "<<(eva00.begin(it)).time_start;<<" time is "<<time<<endl;
							/*cout<<endl<<"point1 == "<<point1<<" but "<<s1.size()<<endl;*/
							//cout<<"-eva01-"<<s1[point1]<<"--";
							for(int i = 0;i<Path[s1[point1]].size();i++){
								if(to == Path[s1[point1]][i])return false;
							}
						}
					it++;
				};
			}
			//cout<<"pes";
			it = 1;
			if(!eva02.empty()){
				while(eva02.order.begin()+it != eva02.order.end()){

						if(eva02.begin(it).time_end > time - (eva02.begin(it)).time_start){
							auto s1 = eva02.begin(it).way;

							int point1 = time - (eva02.begin(it)).time_start;

							//cout<<endl<<"eva00.free_ time << "<<eva00.free_time<<endl;
							/*cout<<endl<<"way "<<s1.size()<<endl;
							cout<<endl<<"it == "<<it<<" but "<<(eva00.begin(it)).time_start<<" time is "<<time<<endl;*/
							//cout<<endl<<"point1 == "<<point1<<" == "<<time<<" - "<< eva02.begin(it).time_start<<" but "<<s1.size()<<endl;

							for(int i = 0;i<Path[s1[point1]].size();i++){
								if(to == Path[s1[point1]][i])return false;
							}
						}
					it++;
				};
			}
			//cout<<"to  ";
		}
		//cout<<"TRUE ";
		return true;
	};

	vector <int> find(int v1, int v2, int num, int time, int n=1000){
			vector<int> data = {};
			vector<int> d (n, INF);
			d[v1] = 0;
			vector<int> id (n, 0);
			deque<int> q;
			q.push_back (v1);
			vector<int> p (n, -1);
			int timer = max(tick, time);
			map <int,int> voiderEx;

			while (!q.empty())
			{
				int v = q.front();
				q.pop_front();
				id[v] = 1;
				for (int i=0; i<Path[v].size(); i++){
					int to = Path[v][i]; int len = 1;
					//cout<<"n"<<to<<" "<<v<<endl;
					if(can_use(to, timer, num)){
						//cout<<"GO "<<to<<" "<<v<<endl;
						if (d[to] > d[v] + len)
						{
							d[to] = d[v] + len;
							if (id[to] == 0)
							{	q.push_back (to);}
							else if (id[to] == 1)
							{ q.push_front (to);}
							//cout<<to<<" :"<<p[to]<<"<<--"<<v<<endl;
							p[to] = v;
							id[to] = 1;
							timer++;
						}
					}else if(Path[v].size() <= 2){
						voiderEx[Path[v][i]]++;
						//cout<<"STAY "<<v<<" to "<<timer<<endl;
						i--;
						timer++;
					}
				}
			}
			//cout<<"FF";
			int j = v2;
			while (j != v1) {
				data.push_back(j);
				if(voiderEx[j] != 0){
					for(int i=0;i<voiderEx[j];i++){
						data.push_back(j);
					}
				}
				//cout<<j<<" ";
				j = p[j];
			}
			data.push_back(j);
			//cout<<"RETURN";
			return data;
	};

	CAR* nearest(int point){
		if(eva00.pos == point and eva00.free)return &eva00;
		if(eva01.pos == point and eva01.free)return &eva01;
		if(eva02.pos == point and eva02.free)return &eva02;
		queue <int> q;
		q.push(point);
		bool used[100]={false};

			while(!q.empty()){
					int vs = q.front();
					q.pop();
					for(int i=0;i<Path[vs].size();i++){
						int to = Path[vs][i];
						if(!used[to]){
							q.push(to);
							used[to] = true;
							//cout<<to<<" ";
							if(eva00.pos == to and eva00.free){
								return &eva00;
							}else
							if(eva01.pos == to and eva01.free){
								return &eva01;
							}else
							if(eva02.pos == to and eva02.free){
								return &eva02;
							}
					}
				}
			}

			//cout<<"NO FREE CARS "<<endl;

			q.push(point);
			bool used2[100]={false};

			while(!q.empty()){
				int vs = q.front();
				q.pop();
				for(int i=0;i<Path[vs].size();i++){
					int to = Path[vs][i];
					if(!used2[to]){
						q.push(to);
						used2[to] = true;
						//cout<<to<<" ";

						if(eva00.end == to ){
							return &eva00;
						}else
						if(eva01.end == to){
							return &eva01;
						}else
						if(eva02.end == to){
							return &eva02;
						}


					}
				}

		}
		cout<<"INVALID CAR COORD ERROR"<<endl;
		return &eva00;
	}
	void updata(){
		if(!eva00.empty()){
			//cout<<"Change eva00 on tick "<<tick<<" from "<<eva00.pos<<" to ";
			eva00.pos = eva00.next(tick);
			//cout<<eva00.pos;
		}
		if(!eva01.empty()){
			eva01.pos = eva01.next(tick);
		}
		if(!eva02.empty()){
			eva02.pos = eva02.next(tick);
		}
	}

	vector<pair<int, int>> Ticks(vector<pair<int, int>> v){
		pair <int, int> p0, p1, p2;
		p0.first = eva00.pos;
		p1.first = eva01.pos;
		p2.first = eva02.pos;
    cleaner();
    for(int i =0; i<v.size();i++){
      auto p = v[i];
			auto* lc = nearest(p.first);
			cout<<"ORDER "<<p.first<<" "<< p.second<<" car:"<< (*lc).num<< "  ";
			auto part1 = find(p.first, (*lc).end, (*lc).num, (*lc).free_time);
			//cout<<" ALIVE ";
			auto part2 = find(p.second, p.first, (*lc).num, part1.size());

			for (size_t i = 0; i < part1.size(); i++) {
				cout<<part1[i]<<" ";
			}
			for (size_t i = 0; i < part2.size(); i++) {
				part1.push_back(part2[i]);
				cout<<part2[i]<<" ";
			}
			auto ord = ORDER(p.first, p.second, part1, max((*lc).free_time, tick), max((*lc).free_time, tick) + part1.size());
			(*lc).push(ord);
    }
		updata();
		p0.second = eva00.pos;
		p1.second = eva01.pos;
		p2.second = eva02.pos;



/*-----------------------------------------------------------------------------*/
		tick++;
		return {p0, p1, p2};
	}



	void clear(){
		deque <vector<int>> order_list = {};CAR eva01 = CAR(1, 36, 1);CAR eva02 = CAR(2, 43, 2);CAR eva00 = CAR(3, 37, 3);int tick = 0;vector<vector<int>> Path = {	vector <int> {},	vector <int> { 9, 2 },	vector <int> { 1, 3 },	vector <int> { 2, 11, 4 },	vector <int> { 3, 12, 5 },	vector <int> { 4, 6 },	vector <int> { 5, 13, 7 },	vector <int> { 6, 14, 8 },	vector <int> { 7, 15 },	vector <int> { 1, 16 },	vector <int> { 11 },	vector <int> { 3, 10, 19},	vector <int> { 4 },	vector <int> { 6, 21 },	vector <int> { 7, 22 },	vector <int> { 8, 23 },	vector <int> { 9, 17, 24 },	vector <int> { 16, 18 },	vector <int> { 17, 19 },	vector <int> { 18, 26, 20, 11 },	vector <int> { 19 },	vector <int> { 13, 28 },	vector <int> { 14, 29 },	vector <int> { 15, 31 },	vector <int> { 16, 32 },	vector <int> { 26 },	vector <int> { 19, 25, 27, 35 },	vector <int> { 26, 28 },	vector <int> { 21, 27 },	vector <int> { 22, 38 },	vector <int> { 31 },	vector <int> { 30, 23, 40 },	vector <int> { 24, 41, 33 },	vector <int> { 32, 34 },	vector <int> { 33, 35 },	vector <int> { 34, 26, 36, 42 },	vector <int> { 35 },	vector <int> { 38 },	vector <int> { 37, 29, 39, 44 },	vector <int> { 38, 40 },	vector <int> { 31, 46 },	vector <int> { 32, 47 },	vector <int> { 35, 49 },	vector <int> { 51 },	vector <int> { 38, 53},	vector <int> { 46 },	vector <int> { 45, 54, 40 },	vector <int> { 41, 48 },	vector <int> { 47, 49 },	vector <int> { 42, 48, 50 },	vector <int> { 49, 51 },	vector <int> { 50, 52, 43 },	vector <int> { 51, 53 },	vector <int> { 52, 44, 54 },	vector <int> { 46, 53 },	vector <int> {}};
	}
};

/*
	HOW IT WORKS --> type* _name_; <--make link; _name_ = &_2name_; <--copy link;
	CAR* ph(CAR* a){
	  return a;
	}
	CAR eva01 = CAR(1, 36);
	auto* b = ph(&eva01);
	eva01.pos = 90;
	cout<<(*b).pos;
*/

MAP m = MAP();

void START(){
	m.clear();
	m.cars();
}

string gone(int v, int n, int from, int to){
	if(from == to)return"S ";
	if(v ==9 and n == 1 or n == 9 and v == 1)return"F ";
	if(v ==47 and n == 41 or n == 47 and v == 41)return"F ";
	if(v ==8 and n == 15 or n == 8 and v == 15)return"F ";
	if(v ==46 and n == 54 or n == 46 and v == 54)return"F ";
	if(v==n and from != to)return "F ";
	int i;
	if((v + 1)%4 == n)return"R ";
	if(v == (n + 1) % 4)return"L ";

}

string next(bool use, int from, int to){
	cout<<endl;
	if(m.eva00.free)cout<<"eva00 free; ";
	if(m.eva01.free)cout<<"eva01 free; ";
	if(m.eva02.free)cout<<"eva02 free; ";
	cout<<endl;
	cout<<"On tick "<<m.tick<<" : "<<endl;
	cout<<"eva00 >> "<<m.eva00.pos<<" >> "<<m.eva00.ziel<<endl;
	cout<<"eva01 >> "<<m.eva01.pos<<" >> "<<m.eva01.ziel<<endl;
	cout<<"eva02 >> "<<m.eva02.pos<<" >> "<<m.eva02.ziel<<endl;
	cout<<"--------------------------------"<<endl<<endl;


	string ans = "";
	vector<pair<int, int>> result;
	if(use){result = m.Ticks({{from, to}});}else{
		result = m.Ticks({});
	}
	//out<<"RES : "<<result[0]<<" "<<result[1]<<" "<<result[2]<<endl;
	//cout<<"Coplete s  ";
	cout<<"0>> "<<result[0].first<<" to "<<result[0].second<<endl;
	cout<<"1>> "<<result[1].first<<" to "<<result[1].second<<endl;
	cout<<"2>> "<<result[2].first<<" to "<<result[2].second<<endl;
	/*
	0 - north
	1 = ost
	2 = west
	3 = south
	*/
	int ss0 = result[0].second - result[0].first;
	int v0 = m.eva00.ziel;
//cout<<"TICK eva00 "<<m.eva00.ziel<<endl;

	if(abs(ss0) == 1){
		if(ss0 > 0){
				//ost
				ans += gone(v0, 1, result[0].second, result[0].first);
				if(ans[0] != 'S')m.eva00.ziel = 1;
			}else{
				//west
				ans += gone(v0, 3,result[0].second,  result[0].first);
				if(ans[0] != 'S')m.eva00.ziel = 3;
			}
	}else{
		if(ss0 < 0){
				//north
				ans += gone(v0, 0, result[0].second, result[0].first);
				if(ans[0] != 'S')m.eva00.ziel = 0;
			}else{
				//south
				ans += gone(v0, 2, result[0].second, result[0].first);
				if(ans[0] != 'S')m.eva00.ziel = 2;
			}
		}
	ss0 = result[1].second - result[1].first;
	v0 = m.eva01.ziel;
	if(abs(ss0) == 1){
		if(ss0 > 0){
				//ost
				ans += gone(v0, 1, result[1].second, result[1].first);
				if(ans[2] != 'S')m.eva01.ziel = 1;
			}else{
				//west
				ans += gone(v0, 3, result[1].second, result[1].first);
				if(ans[2] != 'S')m.eva01.ziel = 3;
			}
	}else{
		if(ss0 < 0){
				//north
				ans += gone(v0, 0, result[1].second, result[1].first);
				if(ans[2] != 'S')m.eva01.ziel = 0;
			}else{
				//south
				ans += gone(v0, 2, result[1].second, result[1].first);
				if(ans[2] != 'S')m.eva01.ziel = 2;
			}
		}

	ss0 = result[2].second - result[2].first;
	v0 = m.eva02.ziel;
	if(abs(ss0) == 1){
		if(ss0 > 0){
				//ost
				ans += gone(v0, 1, result[2].second, result[2].first);
				if(ans[4] != 'S')m.eva02.ziel = 1;
			}else{
				//west
				ans += gone(v0, 3, result[2].second, result[2].first);
				if(ans[4] != 'S')m.eva02.ziel = 3;
			}
	}else{
		if(ss0 < 0){
				//north
				ans += gone(v0, 0, result[2].second, result[2].first);
				if(ans[4] != 'S')m.eva02.ziel = 0;
			}else{
				//south
				ans += gone(v0, 2, result[2].second, result[2].first);
				if(ans[4] != 'S')m.eva02.ziel = 2;
			}
		}
	return ans;
}

extern "C"
int find_path(){

	START();



	    /*while(1){
	    	bool esp = false;
	    	string line;
	    	while (!esp)
	       {
		   		//wait when in new.txt will be 1
	    		ifstream in ("new.txt");
	        	line="";
				while(getline(in, line)){
					esp = line=="1" or line=="2";
				}
				in.close();

	        }
		    if(line == "2"){
		    	ifstream orderFile("order.txt");
			    int to, from;
			    orderFile>>to>>from;
			    //cout<<to<<" "<<from;
			    string ans = next(true, from ,to);
			    orderFile.close();
				ofstream a;
				a.open("ans.txt");
				a<<ans;
				a.close();

			}else{
				string ans = next(false, 0, 0);
				ofstream a;
				a.open("ans.txt");
				a<<ans;
				a.close();
			}
			ofstream zero;
	    	zero.open("new.txt");
	    	zero<<"0";
	    	zero.close();
		}*/



	cout<<next(true, 25, 13)<<endl;
	cout<<next(false, 0 ,0)<<endl;
	cout<<next(false, 25, 12)<<endl;


	return 0;
}
