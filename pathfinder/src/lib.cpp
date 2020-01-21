#include "cores.h"
//using namespace std;

int main(){
	setLogger("LOG.txt");
	LOGGER<<"begin\n";
	CORES a = CORES();
	a.CreateConstantMap(1, {2, 11});
	a.CreateConstantMap(2, {1, 12});
	a.CreateConstantMap(11, {1, 12});
	a.CreateConstantMap(12, {2, 11});
	if (a.getTime() == 0){
		LOGGER<<"CORE CREATED\n";
		a.setLogger(&LOGGER);
		a.log("Logger works good\n");
		a.setCars({{1, 0}});
		
		a.addOrder(1, 12);
		a.addOrder(12, 1);
		for (int i = 0; i < 10; ++i)
		{
			cout<<a.next()<<endl;
		}

	}else{
		LOGGER<<"Unknown Fatal Error\n";
		//LOGGER<<(*(a.getMap()))[0];
	}
	return 0;
}
