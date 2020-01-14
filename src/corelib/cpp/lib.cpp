#include "stdc++.h"
#define v1001 vector<int> a[1001];
using namespace std;
// raw constants 
const int N = 1e4 + 100;
//here write map
/*rules ::
	11-12-13-14-15-16-17-18-....-1n   maybe 0, not 1 like 1-2-3
	|  |  |  |  |  |  |  |        |                        |
	21-22-23-24-25-26-27-28-....-2n                       11-...
	|  |  |  |  \  \   \         |
	...           ...
	|  |  |  |  |  |  |  |       |
	m1-m2-m3-m4-m5-m6-m7-m8-....-mn

	you should use CreateConstantMap(index, vector<int>) to set <vector <int> > MAPP[i] = {...}
*/
vector<int> MAPP[1001];

void CreateConstantMap(int i, vector<int> v) {
	MAPP[i] = v;
}

set <pair<int, int>> DIFFDCT;

/*

  MOVEMENTS DCT

  S == STOP
  R == RIGHT
  L == LEFT
  A == 180 round
  F == Forward

  RF == RIGHT + FORWARD
  LF == LEFT + FORWARD
  AF == 180 r + FORWARD

*/


ofstream LOGGER;
void setLogger(string name) {
	LOGGER.open(name);
	//LOGGER;
}

struct MOVE
{
	int pos;
	int view;
	int time;
	int rotationDgr;
	string Rotation;
};

vector<int>* getTotalMap() {
	return MAPP;
}


struct ORDER {

	int carId;
	int from = 1;
	int to = 1;
	deque<int> way = {};
	vector <MOVE> movements;
	int time_start = 0;
	int time_end = 0;

	void create_movements(int vw = 0) {
		int fp = from;
		for (int i = 0; i < way.size(); i++) {
			MOVE x;

			movements.push_back(x);
		}
	}
	string getMovements(int time) {
		return movements[time - time_start].Rotation;
	}
	int getMovementsDgr(int time) {
		return movements[time - time_start].rotationDgr;
	}

	ORDER(int from_, int to_, int _car, deque<int> way_, int time_start_) {
		from = from_;
		to = to_;
		way = way_;
		//create_movements();
		time_start = time_start_;
		carId = _car;
	}
};

struct CAR
{
	int lastPos;
	string command = "S";
	int pos;
	int FutureEndPos;
	int id = 0;
	int viewDgr = 0; //
	string view;
	bool use = false;
	deque <ORDER> orders = {};

	CAR(int a, int v, int nu) {
		id = nu; viewDgr = v; pos = a;
		FutureEndPos = pos;
		view = "NONE";
		//v%=360;
		switch (v) {
		case 0:
			view = "NORTH";
			return;
		case 90:
			view = "EAST";
			return;
		case 180:
			view = "SOUTH";
			return;
		case 270:
			view = "WEST";
			return;
		}


	}
	/*CAR(int a, int v){
		id = 0; viewDgr = v; pos = a;
		v%=360;
		switch (v){
			case 0:
				view = "NORTH";
				break;
			case 90:
				view = "EAST";
				break;
			case 180:
				view = "SOUTH";
				break;
			case 270:
				view = "WEST";
				break;
		}
	}*/

	void detectCommand(int last, int view) {
		//cout<<last-view<<"COOMANDO/n"<<endl;
		switch (view - last) {
		case 0:
			command = "F";
			break;
		case 90:
			command = "R";
			break;
		case 180:
			command = "A";
			break;
		case 270:
			command = "L";
			break;
		case -90:
			command = "L";
			break;
		case -180:
			command = "A";
			break;
		case -270:
			command = "R";
			break;
		}
	}


	void next() {
		lastPos = pos;
		if (orders.size() == 0) { use = false; FutureEndPos = pos; command = "S"; return; }
		if (orders.front().way.size() == 0) {
			orders.pop_front();
			return;
		}
		//moves

		int newPos = orders.front().way.back();
		int delta = newPos - pos;
		//cout<<" DELTA:: "<<delta<<endl;
		int lastView = viewDgr;
		if (DIFFDCT.find({ pos, newPos }) != DIFFDCT.end()) {
			command = "F";
		}
		else if (delta == 1) {
			viewDgr = 90;
			detectCommand(lastView, viewDgr);
		}
		else if (delta == -1) {
			viewDgr = 270;
			detectCommand(lastView, viewDgr);
		}
		else if (delta == -10) {
			viewDgr = 0;
			detectCommand(lastView, viewDgr);
		}
		else if (delta == 10) {
			viewDgr = 180;
			detectCommand(lastView, viewDgr);
		}
		else if (delta == 0) {
			command = "S";
		}
		//viewDgr %= 360;
		switch (viewDgr) {
		case 0:
			view = "NORTH";
			break;
		case 90:
			view = "EAST";
			break;
		case 180:
			view = "SOUTH";
			break;
		case 270:
			view = "WEST";
			break;
		}

		pos = orders.front().way.back();
		orders.front().way.pop_back();

		//clear some trash
		if (orders.front().way.size() == 0) {
			orders.pop_front();
		}
		if (orders.size() == 0) { use = false; FutureEndPos = pos; }
	}
};


struct CORES {
	ofstream* logger;
	ifstream input;
	int carsCount = 0;
	vector <CAR> carList;
	vector<int> Path[1001];
	vector<int> PathLocal[1001];
	int tick = 0;

	int getTime() { return tick; }

	vector<int>* getMap() {
		return Path;
	}

	void setLogger(ofstream* oft) {
		logger = oft;
		(*logger) << "LOGGER COMPLETED\n";
	}

	void CreateConstantMap(int i, vector<int> v) {
		Path[i] = v;
	}

	template<class T>
	void log(T a) {
		(*logger) << a;
	}

	bool can_use() { return true; }

	deque <int> find(int v1, int v2, int num, int time, int n = 1000) {
		deque<int> data = {};
		vector<int> d(n, N);
		d[v1] = 0;
		vector<int> id(n, 0);
		deque<int> q;
		q.push_back(v1);
		vector<int> p(n, -1);
		int timer = max(tick, time);
		map <int, int> voiderEx;

		while (!q.empty())
		{
			int v = q.front();
			q.pop_front();
			id[v] = 1;
			for (int i = 0; i < Path[v].size(); i++) {
				int to = Path[v][i]; int len = 1;
				//cout<<"n"<<to<<" "<<v<<endl;
				if (can_use()) {
					//cout<<"GO "<<to<<" "<<v<<endl;
					if (d[to] > d[v] + len)
					{
						d[to] = d[v] + len;
						if (id[to] == 0)
						{
							q.push_back(to);
						}
						else if (id[to] == 1)
						{
							q.push_front(to);
						}
						//cout<<to<<" :"<<p[to]<<"<<--"<<v<<endl;
						p[to] = v;
						id[to] = 1;
						timer++;
					}
				}
				else if (Path[v].size() <= 2) {
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
			if (voiderEx[j] != 0) {
				for (int i = 0; i < voiderEx[j]; i++) {
					data.push_back(j);
				}
			}
			//cout<<j<<" ";
			j = p[j];
		}
		data.push_back(j);
		//cout<<"RETURN";

		return data;
	}

	void setCars(vector<pair<int, int>> cars) {
		carsCount = cars.size();
		for (int i = 0; i < cars.size(); i++) {
			carList.push_back(CAR(cars[i].first, cars[i].second, i));
			cout << i << " " << carList[i].viewDgr << endl;
		}
		log("Car poses setted \n");
	}

	int findNearestCar(int pos) {
		for (int i = 0; i < carList.size(); i++) {
			if (carList[i].FutureEndPos == pos)return i;
		}
		//cout<<"shit";
		//returns i, i is v[i] in vector of cars.==> v[i]=CAR(v[i].id, v[i].)
		return 0;
	}

	string format(const char* fmt, ...) {

		//not my function from https://habr.com/ru/post/318962/
		va_list args;
		va_start(args, fmt);
		vector<char> v(2056);
		while (true)
		{
			va_list args2;
			va_copy(args2, args);
			int res = vsnprintf(v.data(), v.size(), fmt, args2);
			if ((res >= 0) && (res < static_cast<int>(v.size())))
			{
				va_end(args);
				va_end(args2);
				return std::string(v.data());
			}
			size_t size;
			if (res < 0)
				size = v.size() * 2;
			else
				size = static_cast<size_t>(res) + 1;
			v.clear();
			v.resize(size);
			va_end(args2);
		}
	}

	void addOrder(int posBegin, int posEnd) {
		//find nearest car
		cout << posBegin << " but " << carList[0].pos << endl;
		int choosen = findNearestCar(posBegin);
		carList[choosen].use = true;
		if (carList[choosen].FutureEndPos != posBegin) {
			log("\n---ADD ORDER (secondary)---\n");
			//find way from nearest car to posBegin;
			log(format("from: %d, to: %d \n", carList[choosen].pos, posBegin));
			log(format("car num: %d, car pos is: %d, car view is: %d \n", choosen, carList[choosen].pos, carList[choosen].viewDgr));

		}

		auto thisPath = find(posBegin, posEnd, choosen, tick);
		carList[choosen].orders.push_back(ORDER(posBegin, posEnd, choosen, thisPath, tick));
		carList[choosen].FutureEndPos = thisPath[0];

		log("\n---ADD ORDER---\n");
		log(format("from: %d, to: %d \n", posBegin, posEnd));
		log(format("car num: %d, car pos is: %d, car view is: %d\n", choosen, carList[choosen].pos, carList[choosen].viewDgr));
		string s = "";
		for (int i = 0; i < thisPath.size(); i++) { s += format("%d ", thisPath[i]); }
		s += "\n";
		log(s);

	}
	string ANS;

	string next() {
		ANS = "";
		log(format("\n\n\n---TICK %d INFORMATION::: \n ", tick));
		// choose new orders
		for (int i = 0; i < carList.size(); i++) {
			carList[i].next();
			ANS += carList[i].command + " ";
			log(format("car %d :\n    lastPos: %d; NewPos: %d; Coomand: ", i, carList[i].lastPos, carList[i].pos) + carList[i].command + "\n");
		}

		tick++;
		return ANS;
	}

	CORES() {
		tick = 0;
		carsCount = 0;
		for (int i = 0; i < 100; i++) {
			Path[i] = {};
		}
	}
};