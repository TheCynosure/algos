#include <stdlib.h>
#include <stdio.h>
//For now this circular queue will just store strings.
struct Node {
	char *data;
	struct Node *next;	
};

struct CircQ {
	struct Node *rear;
	int size;
};

typedef struct Node Node;
typedef struct CircQ CircQ;

struct CircQ *init_q() {
	CircQ *cq = malloc(sizeof(struct CircQ));
	cq->size = 0;
	cq->rear = NULL;
	return cq;
}

void enqueue(CircQ *cq, char *data) {
	Node *node = malloc(sizeof(Node));
	node->data = data;
	if(cq->rear == NULL) {
		cq->rear = node;
		node->next = node;
	} else {
		node->next = cq->rear->next;
		cq->rear->next = node;
		cq->rear = node;
	}
	cq->size++;
}

char *dequeue(CircQ *cq) {
	if(cq->size == 0)
		return NULL;
	if(cq->size == 1) {
		char *data = cq->rear->data;
		free(cq->rear);
		cq->rear = NULL;
		cq->size--;
		return data;
	} else {
		char *data = cq->rear->next->data;
		void *new_front = cq->rear->next->next;
		free(cq->rear->next);
		cq->rear->next = new_front;
		cq->size--;
		return data;
	}
}

void free_q(CircQ *cq) {
	Node *prev = NULL;
	for(Node *curr = cq->rear; curr != NULL; curr = curr->next) {
		if(prev != NULL) {
			free(prev);
		}
		prev = curr;
	}
	free(prev);
	free(cq);
}

int main(int argc, char** argv) {
	CircQ *cq = init_q();
	enqueue(cq, "Hello");
	enqueue(cq, "There");
	printf("%s", dequeue(cq));
	printf("%s", dequeue(cq));
	free_q(cq);
	return 0;
}