#[doc = "Register `SGPIO004` reader"]
pub type R = crate::R<Sgpio004Spec>;
#[doc = "Register `SGPIO004` writer"]
pub type W = crate::W<Sgpio004Spec>;
#[doc = "Field `InputValueOfSGPIO0` reader - Input value of SGPIO_0"]
pub type InputValueOfSgpio0R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO1` reader - Input value of SGPIO_1"]
pub type InputValueOfSgpio1R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO2` reader - Input value of SGPIO_2"]
pub type InputValueOfSgpio2R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO3` reader - Input value of SGPIO_3"]
pub type InputValueOfSgpio3R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO4` reader - Input value of SGPIO_4"]
pub type InputValueOfSgpio4R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO5` reader - Input value of SGPIO_5"]
pub type InputValueOfSgpio5R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO6` reader - Input value of SGPIO_6"]
pub type InputValueOfSgpio6R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO7` reader - Input value of SGPIO_7"]
pub type InputValueOfSgpio7R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO8` reader - Input value of SGPIO_8"]
pub type InputValueOfSgpio8R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO9` reader - Input value of SGPIO_9"]
pub type InputValueOfSgpio9R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO10` reader - Input value of SGPIO_10"]
pub type InputValueOfSgpio10R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO11` reader - Input value of SGPIO_11"]
pub type InputValueOfSgpio11R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO12` reader - Input value of SGPIO_12"]
pub type InputValueOfSgpio12R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO13` reader - Input value of SGPIO_13"]
pub type InputValueOfSgpio13R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO14` reader - Input value of SGPIO_14"]
pub type InputValueOfSgpio14R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO15` reader - Input value of SGPIO_15"]
pub type InputValueOfSgpio15R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO16` reader - Input value of SGPIO_16"]
pub type InputValueOfSgpio16R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO17` reader - Input value of SGPIO_17"]
pub type InputValueOfSgpio17R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO18` reader - Input value of SGPIO_18"]
pub type InputValueOfSgpio18R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO19` reader - Input value of SGPIO_19"]
pub type InputValueOfSgpio19R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO20` reader - Input value of SGPIO_20"]
pub type InputValueOfSgpio20R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO21` reader - Input value of SGPIO_21"]
pub type InputValueOfSgpio21R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO22` reader - Input value of SGPIO_22"]
pub type InputValueOfSgpio22R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO23` reader - Input value of SGPIO_23"]
pub type InputValueOfSgpio23R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO24` reader - Input value of SGPIO_24"]
pub type InputValueOfSgpio24R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO25` reader - Input value of SGPIO_25"]
pub type InputValueOfSgpio25R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO26` reader - Input value of SGPIO_26"]
pub type InputValueOfSgpio26R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO27` reader - Input value of SGPIO_27"]
pub type InputValueOfSgpio27R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO28` reader - Input value of SGPIO_28"]
pub type InputValueOfSgpio28R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO29` reader - Input value of SGPIO_29"]
pub type InputValueOfSgpio29R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO30` reader - Input value of SGPIO_30"]
pub type InputValueOfSgpio30R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO31` reader - Input value of SGPIO_31"]
pub type InputValueOfSgpio31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input value of SGPIO_0"]
    #[inline(always)]
    pub fn input_value_of_sgpio0(&self) -> InputValueOfSgpio0R {
        InputValueOfSgpio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input value of SGPIO_1"]
    #[inline(always)]
    pub fn input_value_of_sgpio1(&self) -> InputValueOfSgpio1R {
        InputValueOfSgpio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input value of SGPIO_2"]
    #[inline(always)]
    pub fn input_value_of_sgpio2(&self) -> InputValueOfSgpio2R {
        InputValueOfSgpio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input value of SGPIO_3"]
    #[inline(always)]
    pub fn input_value_of_sgpio3(&self) -> InputValueOfSgpio3R {
        InputValueOfSgpio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input value of SGPIO_4"]
    #[inline(always)]
    pub fn input_value_of_sgpio4(&self) -> InputValueOfSgpio4R {
        InputValueOfSgpio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input value of SGPIO_5"]
    #[inline(always)]
    pub fn input_value_of_sgpio5(&self) -> InputValueOfSgpio5R {
        InputValueOfSgpio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input value of SGPIO_6"]
    #[inline(always)]
    pub fn input_value_of_sgpio6(&self) -> InputValueOfSgpio6R {
        InputValueOfSgpio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input value of SGPIO_7"]
    #[inline(always)]
    pub fn input_value_of_sgpio7(&self) -> InputValueOfSgpio7R {
        InputValueOfSgpio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input value of SGPIO_8"]
    #[inline(always)]
    pub fn input_value_of_sgpio8(&self) -> InputValueOfSgpio8R {
        InputValueOfSgpio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input value of SGPIO_9"]
    #[inline(always)]
    pub fn input_value_of_sgpio9(&self) -> InputValueOfSgpio9R {
        InputValueOfSgpio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input value of SGPIO_10"]
    #[inline(always)]
    pub fn input_value_of_sgpio10(&self) -> InputValueOfSgpio10R {
        InputValueOfSgpio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input value of SGPIO_11"]
    #[inline(always)]
    pub fn input_value_of_sgpio11(&self) -> InputValueOfSgpio11R {
        InputValueOfSgpio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input value of SGPIO_12"]
    #[inline(always)]
    pub fn input_value_of_sgpio12(&self) -> InputValueOfSgpio12R {
        InputValueOfSgpio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input value of SGPIO_13"]
    #[inline(always)]
    pub fn input_value_of_sgpio13(&self) -> InputValueOfSgpio13R {
        InputValueOfSgpio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input value of SGPIO_14"]
    #[inline(always)]
    pub fn input_value_of_sgpio14(&self) -> InputValueOfSgpio14R {
        InputValueOfSgpio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input value of SGPIO_15"]
    #[inline(always)]
    pub fn input_value_of_sgpio15(&self) -> InputValueOfSgpio15R {
        InputValueOfSgpio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input value of SGPIO_16"]
    #[inline(always)]
    pub fn input_value_of_sgpio16(&self) -> InputValueOfSgpio16R {
        InputValueOfSgpio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input value of SGPIO_17"]
    #[inline(always)]
    pub fn input_value_of_sgpio17(&self) -> InputValueOfSgpio17R {
        InputValueOfSgpio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input value of SGPIO_18"]
    #[inline(always)]
    pub fn input_value_of_sgpio18(&self) -> InputValueOfSgpio18R {
        InputValueOfSgpio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input value of SGPIO_19"]
    #[inline(always)]
    pub fn input_value_of_sgpio19(&self) -> InputValueOfSgpio19R {
        InputValueOfSgpio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input value of SGPIO_20"]
    #[inline(always)]
    pub fn input_value_of_sgpio20(&self) -> InputValueOfSgpio20R {
        InputValueOfSgpio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input value of SGPIO_21"]
    #[inline(always)]
    pub fn input_value_of_sgpio21(&self) -> InputValueOfSgpio21R {
        InputValueOfSgpio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input value of SGPIO_22"]
    #[inline(always)]
    pub fn input_value_of_sgpio22(&self) -> InputValueOfSgpio22R {
        InputValueOfSgpio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input value of SGPIO_23"]
    #[inline(always)]
    pub fn input_value_of_sgpio23(&self) -> InputValueOfSgpio23R {
        InputValueOfSgpio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input value of SGPIO_24"]
    #[inline(always)]
    pub fn input_value_of_sgpio24(&self) -> InputValueOfSgpio24R {
        InputValueOfSgpio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input value of SGPIO_25"]
    #[inline(always)]
    pub fn input_value_of_sgpio25(&self) -> InputValueOfSgpio25R {
        InputValueOfSgpio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input value of SGPIO_26"]
    #[inline(always)]
    pub fn input_value_of_sgpio26(&self) -> InputValueOfSgpio26R {
        InputValueOfSgpio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input value of SGPIO_27"]
    #[inline(always)]
    pub fn input_value_of_sgpio27(&self) -> InputValueOfSgpio27R {
        InputValueOfSgpio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input value of SGPIO_28"]
    #[inline(always)]
    pub fn input_value_of_sgpio28(&self) -> InputValueOfSgpio28R {
        InputValueOfSgpio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input value of SGPIO_29"]
    #[inline(always)]
    pub fn input_value_of_sgpio29(&self) -> InputValueOfSgpio29R {
        InputValueOfSgpio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input value of SGPIO_30"]
    #[inline(always)]
    pub fn input_value_of_sgpio30(&self) -> InputValueOfSgpio30R {
        InputValueOfSgpio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Input value of SGPIO_31"]
    #[inline(always)]
    pub fn input_value_of_sgpio31(&self) -> InputValueOfSgpio31R {
        InputValueOfSgpio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial In Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio004Spec;
impl crate::RegisterSpec for Sgpio004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio004::R`](R) reader structure"]
impl crate::Readable for Sgpio004Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio004::W`](W) writer structure"]
impl crate::Writable for Sgpio004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO004 to value 0"]
impl crate::Resettable for Sgpio004Spec {}
