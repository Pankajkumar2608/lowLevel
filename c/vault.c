#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>


struct Entry
{
    char *key;  // why we not put a value like char key[100] --- answer -- we do not know size of value put by user if put large number than there will be waste of memory;
    char *value; 
    struct Entry *next; //points to the same struct of same type;;; creates a chain in memory;
};


char *safe_strdup(const char *src){
    int len = strlen(src); // return a length of string
    char *dest = malloc(len+1); // point a address greater than the length of string;

    // checking if dest created or not if somehow it failed and we not change the it will crash the program
    if(!dest){
        perror ("Memory allocation faild");
        exit(1);
    }

    strcpy(dest,src);  // its copys the string it basically takes two arg one destination and other one is src which thing we want to copy
    return dest;
}

void add_entry(struct Entry **head,const char *k, const char *v){
    struct Entry *new_node = malloc(sizeof(struct Entry));

    if(!new_node) return;

    new_node->key = safe_strdup(k);
    new_node->value = safe_strdup(v);

    new_node -> next = *head;
    *head = new_node;

}
void free_database(struct Entry *head){
    struct Entry *tmp;
    while(head != NULL){
        tmp = head;
        head = head->next;

        free(tmp-> key);
        free(tmp->value);
        free(tmp);
    }
}

void print_database(struct Entry *head){
    printf("/n ----vault contents ---- /n");
    while( head != NULL){
        printf("%s: %s/n", head->key,head->value);

    }
    printf("---------------/n");
}

void save_to_file(struct Entry *head, const char *filename){
    int fd = open(filename, O_WRONLY| O_CREAT | O_TRUNC, 0664);
    if(fd < 0){
        perror("failed to open file");
        return;
    }
    struct Entry *current = head;
    while(current != NULL){
        int k_len = strlen(current->key);
        int v_len = strlen(current -> value);

        write (fd, &k_len, sizeof(int));
        write(fd, current ->key, k_len);
        write(fd, &v_len, sizeof(int));
        write(fd,current ->value, v_len);
        
        current = current ->next;
    };
    close(fd);
    printf("Database saved to %s\n", filename);
}


void load_from_file(struct Entry **head, const char *filename){
    int fd = open(filename, O_RDONLY);
    if(fd < 0 ) return;

    int k_len, v_len;

    while(read(fd,&k_len, sizeof(int))>0){
        char *k_buf = malloc(k_len + 1);

        read(fd,k_buf,k_len);
        k_buf[k_len] = '\0';
        read(fd, &v_len, sizeof(int));
        char *v_buf = malloc(v_len + 1);
        read(fd, v_buf, v_len);
        v_buf[v_len] = '\0';

        add_entry(head, k_buf, v_buf);

        // Free our temp buffers (add_entry made its own copies)
        free(k_buf);
        free(v_buf);
    }
    close(fd);
}

int main()
{
    // TOPIC: Memory (The head of our list)
    struct Entry *db_head = NULL;

    load_from_file(&db_head, "database.db");

    // Simple interaction loop
    add_entry(&db_head, "username", "admin");
    add_entry(&db_head, "password", "1234");
    add_entry(&db_head, "status", "active");

    print_database(db_head);

    save_to_file(db_head, "database.db");

    // Clean up before exiting
    free_database(db_head);

    return 0;
}
