/* USER CODE BEGIN Header */
/**
 ******************************************************************************
 * @file           : main.h
 * @brief          : Header for main.c file.
 *                   This file contains the common defines of the application.
 ******************************************************************************
 * @attention
 *
 * Copyright (c) 2023 STMicroelectronics.
 * All rights reserved.
 *
 * This software is licensed under terms that can be found in the LICENSE file
 * in the root directory of this software component.
 * If no LICENSE file comes with this software, it is provided AS-IS.
 *
 ******************************************************************************
 */
/* USER CODE END Header */

/* Define to prevent recursive inclusion -------------------------------------*/
#ifndef __MAIN_H
#define __MAIN_H

#ifdef __cplusplus
extern "C" {
#endif

/* Includes ------------------------------------------------------------------*/
#include "stm32f4xx_hal.h"

/* Exported functions prototypes ---------------------------------------------*/
void Error_Handler(void);

/* Private defines -----------------------------------------------------------*/
#define MAX6675_CS_Pin              GPIO_PIN_0
#define MAX6675_CS_GPIO_Port        GPIOB
#define USBD_DM_Pin                 GPIO_PIN_11
#define USBD_DM_GPIO_Port           GPIOA
#define USBD_DP_Pin                 GPIO_PIN_12
#define USBD_DP_GPIO_Port           GPIOA

#define LED_STM32_Pin               GPIO_PIN_13
#define LED_STM32_Port              GPIOC

#define LED_INDICATORB_Pin          GPIO_PIN_3
#define LED_INDICATORA_Pin          GPIO_PIN_4
#define LED_INDICATOR_Port          GPIOB
#define RPM_INPUT_CAPTURE_Pin       GPIO_PIN_9
#define RPM_INPUT_CAPTURE_GPIO_Port GPIOB

/* USER CODE BEGIN Private defines */
#define SET_PIN(PORT, PIN)   ((PORT)->BSRR = (PIN))
#define RESET_PIN(PORT, PIN) ((PORT)->BSRR = (uint32_t)(PIN) << 16U)

#define LED_BLINKS(PORT, PIN, BLINK_COUNT, BLINK_PERIOD_MS)                                        \
    for (int _i = 0; _i < BLINK_COUNT; _i++) {                                                     \
        HAL_Delay(BLINK_PERIOD_MS);                                                                \
        SET_PIN(PORT, PIN);                                                                        \
        HAL_Delay(BLINK_PERIOD_MS);                                                                \
        RESET_PIN(PORT, PIN);                                                                      \
    }

#define DYNO_STARTED          0x0
#define DYNO_STOPPED          0x1

#define MAX6675_NOT_CONNECTED 0x0
#define MAX6675_CONNECTED     0x1

extern SPI_HandleTypeDef hspi_max6675;
extern SPI_HandleTypeDef hspi1;
extern TIM_HandleTypeDef htim4;
void MX_Start(void);
void MX_Stop(void);
/* USER CODE END Private defines */

#ifdef __cplusplus
}
#endif

#endif /* __MAIN_H */
