#ifndef LIST_H
#define LIST_H

#define LIST_DEFAULT_CAPACITY 100
#define LIST_SCALING_FACTOR 1.5

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct list_int {
	int* data;
	int size;
	int capacity;
} list_int;

list_int* list_int_new() {
	list_int* l = malloc(sizeof(list_int));
	l->data = malloc(sizeof(int) * LIST_DEFAULT_CAPACITY);
	l->size = 0;
	l->capacity = LIST_DEFAULT_CAPACITY;

	return l;
}

void list_int_free(list_int* l) {
	free(l->data);
	free(l);
}

void list_int_append(list_int* l, int i) {
	if (l->size == l->capacity) {
		int* upsize = malloc(sizeof(int) * l->capacity * LIST_SCALING_FACTOR);
		memcpy(upsize, l->data, l->size * sizeof(int));
		free(l->data);
		l->data = upsize;
		l->capacity *= LIST_SCALING_FACTOR;
	}

	l->data[l->size++] = i;
}

typedef struct list_float {
	float* data;
	int size;
	int capacity;
} list_float;

list_float* list_float_new() {
	list_float* l = malloc(sizeof(list_float));
	l->data = malloc(sizeof(float) * LIST_DEFAULT_CAPACITY);
	l->size = 0;
	l->capacity = LIST_DEFAULT_CAPACITY;

	return l;
}

void list_float_free(list_float* l) {
	free(l->data);
	free(l);
}

void list_float_append(list_float* l, float i) {
	if (l->size == l->capacity) {
		float* upsize = malloc(sizeof(float) * l->capacity * LIST_SCALING_FACTOR);
		memcpy(upsize, l->data, l->size * sizeof(float));
		free(l->data);
		l->data = upsize;
		l->capacity *= LIST_SCALING_FACTOR;
	}

	l->data[l->size++] = i;
}

#endif /* end of include guard: LIST_H */
