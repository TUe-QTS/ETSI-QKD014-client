#include "config.h"

#include <assert.h>
#include <etsi014-client/etsi014-client.h>
#include <stdio.h>
#include <string.h>

#define KEY_SIZE_BYTES KEY_SIZE_BITS / 8

int main(void)
{
    const ETSI014Client* client = NULL;
    const char* error_str = NULL;
    if (e14_new_etsi014_client(
            HOST, PORT, CERT_1, KEY_1, SERVER_CA, &client, &error_str)) {
        printf("Failed to create etsi014 client: %s\n", error_str);
        e14_free_error_str(&error_str);
        return 1;
    }
    E14_KME_Status status;
    if (e14_get_status(client, SAE_ID_2, &status, &error_str)) {
        printf("Failed to get status: %s\n", error_str);
        e14_free_error_str(&error_str);
        return 1;
    }
    printf("target_sae_id: %s\n", status.target_sae_id);
    const int amount_of_keys = 3;
    E14_QKD_Key keys1[amount_of_keys];
    if (e14_get_keys(client, KEY_SIZE_BITS, SAE_ID_2, NULL, 0, amount_of_keys, keys1,
            &error_str)) {
        printf("Failed to get keys: %s\n", error_str);
        e14_free_error_str(&error_str);
        return 1;
    }
    for (int i = 0; i < amount_of_keys; i++) {
        const E14_QKD_Key key = keys1[i];
        printf("%s\n", key.uuid);
    }
    e14_free_etsi014_client(&client);
    if (e14_new_etsi014_client(
            HOST, PORT, CERT_2, KEY_2, SERVER_CA, &client, &error_str)) {
        printf("Failed to create etsi014 client: %s\n", error_str);
        e14_free_error_str(&error_str);
        return 1;
    }
    char* key_ids[] = { keys1[0].uuid, keys1[1].uuid, keys1[2].uuid };
    E14_QKD_Key keys2[amount_of_keys];
    if (e14_get_keys_by_ids(
            client, SAE_ID_1, key_ids, amount_of_keys, keys2, &error_str)) {
        printf("Failed to get keys: %s\n", error_str);
        e14_free_error_str(&error_str);
        return 1;
    }
    for (int i = 0; i < amount_of_keys; i++) {
        assert(strcmp(keys1[i].uuid, keys2[i].uuid) == 0);
        assert(keys1[i].key_size == KEY_SIZE_BYTES);
        assert(keys2[i].key_size == KEY_SIZE_BYTES);
        const KeyBytesBorrow* borrow1;
        const uint8_t* key_bytes1;
        e14_unprotect_qkd_key_bytes(keys1[i].key_bytes_protected, &borrow1, &key_bytes1);
        const KeyBytesBorrow* borrow2;
        const uint8_t* key_bytes2;
        e14_unprotect_qkd_key_bytes(keys2[i].key_bytes_protected, &borrow2, &key_bytes2);
        // Warning: constant time compare should be used on keys instead of memcmp
        assert(memcmp(key_bytes1, key_bytes2, KEY_SIZE_BYTES) == 0);
        e14_protect_qkd_key_bytes(&borrow1, &key_bytes1);
        e14_protect_qkd_key_bytes(&borrow2, &key_bytes2);
        e14_free_qkd_key_bytes(&keys1[i].key_bytes_protected);
        e14_free_qkd_key_bytes(&keys2[i].key_bytes_protected);
    }
    e14_free_etsi014_client(&client);
    return 0;
}
