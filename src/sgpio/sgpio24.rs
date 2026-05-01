#[doc = "Register `SGPIO24` reader"]
pub type R = crate::R<Sgpio24Spec>;
#[doc = "Register `SGPIO24` writer"]
pub type W = crate::W<Sgpio24Spec>;
#[doc = "Field `OutputValueOfSGPIO0` reader - Output value of SGPIO_0"]
pub type OutputValueOfSgpio0R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO1` reader - Output value of SGPIO_1"]
pub type OutputValueOfSgpio1R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO2` reader - Output value of SGPIO_2"]
pub type OutputValueOfSgpio2R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO3` reader - Output value of SGPIO_3"]
pub type OutputValueOfSgpio3R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO4` reader - Output value of SGPIO_4"]
pub type OutputValueOfSgpio4R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO5` reader - Output value of SGPIO_5"]
pub type OutputValueOfSgpio5R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO6` reader - Output value of SGPIO_6"]
pub type OutputValueOfSgpio6R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO7` reader - Output value of SGPIO_7"]
pub type OutputValueOfSgpio7R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO8` reader - Output value of SGPIO_8"]
pub type OutputValueOfSgpio8R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO9` reader - Output value of SGPIO_9"]
pub type OutputValueOfSgpio9R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO10` reader - Output value of SGPIO_10"]
pub type OutputValueOfSgpio10R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO11` reader - Output value of SGPIO_11"]
pub type OutputValueOfSgpio11R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO12` reader - Output value of SGPIO_12"]
pub type OutputValueOfSgpio12R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO13` reader - Output value of SGPIO_13"]
pub type OutputValueOfSgpio13R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO14` reader - Output value of SGPIO_14"]
pub type OutputValueOfSgpio14R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO15` reader - Output value of SGPIO_15"]
pub type OutputValueOfSgpio15R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO16` reader - Output value of SGPIO_16"]
pub type OutputValueOfSgpio16R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO17` reader - Output value of SGPIO_17"]
pub type OutputValueOfSgpio17R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO18` reader - Output value of SGPIO_18"]
pub type OutputValueOfSgpio18R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO19` reader - Output value of SGPIO_19"]
pub type OutputValueOfSgpio19R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO20` reader - Output value of SGPIO_20"]
pub type OutputValueOfSgpio20R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO21` reader - Output value of SGPIO_21"]
pub type OutputValueOfSgpio21R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO22` reader - Output value of SGPIO_22"]
pub type OutputValueOfSgpio22R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO23` reader - Output value of SGPIO_23"]
pub type OutputValueOfSgpio23R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO24` reader - Output value of SGPIO_24"]
pub type OutputValueOfSgpio24R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO25` reader - Output value of SGPIO_25"]
pub type OutputValueOfSgpio25R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO26` reader - Output value of SGPIO_26"]
pub type OutputValueOfSgpio26R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO27` reader - Output value of SGPIO_27"]
pub type OutputValueOfSgpio27R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO28` reader - Output value of SGPIO_28"]
pub type OutputValueOfSgpio28R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO29` reader - Output value of SGPIO_29"]
pub type OutputValueOfSgpio29R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO30` reader - Output value of SGPIO_30"]
pub type OutputValueOfSgpio30R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO31` reader - Output value of SGPIO_31"]
pub type OutputValueOfSgpio31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Output value of SGPIO_0"]
    #[inline(always)]
    pub fn output_value_of_sgpio0(&self) -> OutputValueOfSgpio0R {
        OutputValueOfSgpio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output value of SGPIO_1"]
    #[inline(always)]
    pub fn output_value_of_sgpio1(&self) -> OutputValueOfSgpio1R {
        OutputValueOfSgpio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output value of SGPIO_2"]
    #[inline(always)]
    pub fn output_value_of_sgpio2(&self) -> OutputValueOfSgpio2R {
        OutputValueOfSgpio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output value of SGPIO_3"]
    #[inline(always)]
    pub fn output_value_of_sgpio3(&self) -> OutputValueOfSgpio3R {
        OutputValueOfSgpio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output value of SGPIO_4"]
    #[inline(always)]
    pub fn output_value_of_sgpio4(&self) -> OutputValueOfSgpio4R {
        OutputValueOfSgpio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output value of SGPIO_5"]
    #[inline(always)]
    pub fn output_value_of_sgpio5(&self) -> OutputValueOfSgpio5R {
        OutputValueOfSgpio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output value of SGPIO_6"]
    #[inline(always)]
    pub fn output_value_of_sgpio6(&self) -> OutputValueOfSgpio6R {
        OutputValueOfSgpio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output value of SGPIO_7"]
    #[inline(always)]
    pub fn output_value_of_sgpio7(&self) -> OutputValueOfSgpio7R {
        OutputValueOfSgpio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output value of SGPIO_8"]
    #[inline(always)]
    pub fn output_value_of_sgpio8(&self) -> OutputValueOfSgpio8R {
        OutputValueOfSgpio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output value of SGPIO_9"]
    #[inline(always)]
    pub fn output_value_of_sgpio9(&self) -> OutputValueOfSgpio9R {
        OutputValueOfSgpio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output value of SGPIO_10"]
    #[inline(always)]
    pub fn output_value_of_sgpio10(&self) -> OutputValueOfSgpio10R {
        OutputValueOfSgpio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output value of SGPIO_11"]
    #[inline(always)]
    pub fn output_value_of_sgpio11(&self) -> OutputValueOfSgpio11R {
        OutputValueOfSgpio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output value of SGPIO_12"]
    #[inline(always)]
    pub fn output_value_of_sgpio12(&self) -> OutputValueOfSgpio12R {
        OutputValueOfSgpio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output value of SGPIO_13"]
    #[inline(always)]
    pub fn output_value_of_sgpio13(&self) -> OutputValueOfSgpio13R {
        OutputValueOfSgpio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output value of SGPIO_14"]
    #[inline(always)]
    pub fn output_value_of_sgpio14(&self) -> OutputValueOfSgpio14R {
        OutputValueOfSgpio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output value of SGPIO_15"]
    #[inline(always)]
    pub fn output_value_of_sgpio15(&self) -> OutputValueOfSgpio15R {
        OutputValueOfSgpio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output value of SGPIO_16"]
    #[inline(always)]
    pub fn output_value_of_sgpio16(&self) -> OutputValueOfSgpio16R {
        OutputValueOfSgpio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output value of SGPIO_17"]
    #[inline(always)]
    pub fn output_value_of_sgpio17(&self) -> OutputValueOfSgpio17R {
        OutputValueOfSgpio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output value of SGPIO_18"]
    #[inline(always)]
    pub fn output_value_of_sgpio18(&self) -> OutputValueOfSgpio18R {
        OutputValueOfSgpio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output value of SGPIO_19"]
    #[inline(always)]
    pub fn output_value_of_sgpio19(&self) -> OutputValueOfSgpio19R {
        OutputValueOfSgpio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output value of SGPIO_20"]
    #[inline(always)]
    pub fn output_value_of_sgpio20(&self) -> OutputValueOfSgpio20R {
        OutputValueOfSgpio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output value of SGPIO_21"]
    #[inline(always)]
    pub fn output_value_of_sgpio21(&self) -> OutputValueOfSgpio21R {
        OutputValueOfSgpio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output value of SGPIO_22"]
    #[inline(always)]
    pub fn output_value_of_sgpio22(&self) -> OutputValueOfSgpio22R {
        OutputValueOfSgpio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output value of SGPIO_23"]
    #[inline(always)]
    pub fn output_value_of_sgpio23(&self) -> OutputValueOfSgpio23R {
        OutputValueOfSgpio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Output value of SGPIO_24"]
    #[inline(always)]
    pub fn output_value_of_sgpio24(&self) -> OutputValueOfSgpio24R {
        OutputValueOfSgpio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Output value of SGPIO_25"]
    #[inline(always)]
    pub fn output_value_of_sgpio25(&self) -> OutputValueOfSgpio25R {
        OutputValueOfSgpio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Output value of SGPIO_26"]
    #[inline(always)]
    pub fn output_value_of_sgpio26(&self) -> OutputValueOfSgpio26R {
        OutputValueOfSgpio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Output value of SGPIO_27"]
    #[inline(always)]
    pub fn output_value_of_sgpio27(&self) -> OutputValueOfSgpio27R {
        OutputValueOfSgpio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Output value of SGPIO_28"]
    #[inline(always)]
    pub fn output_value_of_sgpio28(&self) -> OutputValueOfSgpio28R {
        OutputValueOfSgpio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Output value of SGPIO_29"]
    #[inline(always)]
    pub fn output_value_of_sgpio29(&self) -> OutputValueOfSgpio29R {
        OutputValueOfSgpio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Output value of SGPIO_30"]
    #[inline(always)]
    pub fn output_value_of_sgpio30(&self) -> OutputValueOfSgpio30R {
        OutputValueOfSgpio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Output value of SGPIO_31"]
    #[inline(always)]
    pub fn output_value_of_sgpio31(&self) -> OutputValueOfSgpio31R {
        OutputValueOfSgpio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial Out Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio24Spec;
impl crate::RegisterSpec for Sgpio24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio24::R`](R) reader structure"]
impl crate::Readable for Sgpio24Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio24::W`](W) writer structure"]
impl crate::Writable for Sgpio24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO24 to value 0"]
impl crate::Resettable for Sgpio24Spec {}
