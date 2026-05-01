#[doc = "Register `SGPIO40` reader"]
pub type R = crate::R<Sgpio40Spec>;
#[doc = "Register `SGPIO40` writer"]
pub type W = crate::W<Sgpio40Spec>;
#[doc = "Field `INTStatusOfSGPIO0` reader - Interrupt Status of SGPIO_0"]
pub type IntstatusOfSgpio0R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO1` reader - Interrupt Status of SGPIO_1"]
pub type IntstatusOfSgpio1R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO2` reader - Interrupt Status of SGPIO_2"]
pub type IntstatusOfSgpio2R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO3` reader - Interrupt Status of SGPIO_3"]
pub type IntstatusOfSgpio3R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO4` reader - Interrupt Status of SGPIO_4"]
pub type IntstatusOfSgpio4R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO5` reader - Interrupt Status of SGPIO_5"]
pub type IntstatusOfSgpio5R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO6` reader - Interrupt Status of SGPIO_6"]
pub type IntstatusOfSgpio6R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO7` reader - Interrupt Status of SGPIO_7"]
pub type IntstatusOfSgpio7R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO8` reader - Interrupt Status of SGPIO_8"]
pub type IntstatusOfSgpio8R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO9` reader - Interrupt Status of SGPIO_9"]
pub type IntstatusOfSgpio9R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO10` reader - Interrupt Status of SGPIO_10"]
pub type IntstatusOfSgpio10R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO11` reader - Interrupt Status of SGPIO_11"]
pub type IntstatusOfSgpio11R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO12` reader - Interrupt Status of SGPIO_12"]
pub type IntstatusOfSgpio12R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO13` reader - Interrupt Status of SGPIO_13"]
pub type IntstatusOfSgpio13R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO14` reader - Interrupt Status of SGPIO_14"]
pub type IntstatusOfSgpio14R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO15` reader - Interrupt Status of SGPIO_15"]
pub type IntstatusOfSgpio15R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO16` reader - Interrupt Status of SGPIO_16"]
pub type IntstatusOfSgpio16R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO17` reader - Interrupt Status of SGPIO_17"]
pub type IntstatusOfSgpio17R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO18` reader - Interrupt Status of SGPIO_18"]
pub type IntstatusOfSgpio18R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO19` reader - Interrupt Status of SGPIO_19"]
pub type IntstatusOfSgpio19R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO20` reader - Interrupt Status of SGPIO_20"]
pub type IntstatusOfSgpio20R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO21` reader - Interrupt Status of SGPIO_21"]
pub type IntstatusOfSgpio21R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO22` reader - Interrupt Status of SGPIO_22"]
pub type IntstatusOfSgpio22R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO23` reader - Interrupt Status of SGPIO_23"]
pub type IntstatusOfSgpio23R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO24` reader - Interrupt Status of SGPIO_24"]
pub type IntstatusOfSgpio24R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO25` reader - Interrupt Status of SGPIO_25"]
pub type IntstatusOfSgpio25R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO26` reader - Interrupt Status of SGPIO_26"]
pub type IntstatusOfSgpio26R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO27` reader - Interrupt Status of SGPIO_27"]
pub type IntstatusOfSgpio27R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO28` reader - Interrupt Status of SGPIO_28"]
pub type IntstatusOfSgpio28R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO29` reader - Interrupt Status of SGPIO_29"]
pub type IntstatusOfSgpio29R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO30` reader - Interrupt Status of SGPIO_30"]
pub type IntstatusOfSgpio30R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO31` reader - Interrupt Status of SGPIO_31"]
pub type IntstatusOfSgpio31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of SGPIO_0"]
    #[inline(always)]
    pub fn intstatus_of_sgpio0(&self) -> IntstatusOfSgpio0R {
        IntstatusOfSgpio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of SGPIO_1"]
    #[inline(always)]
    pub fn intstatus_of_sgpio1(&self) -> IntstatusOfSgpio1R {
        IntstatusOfSgpio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of SGPIO_2"]
    #[inline(always)]
    pub fn intstatus_of_sgpio2(&self) -> IntstatusOfSgpio2R {
        IntstatusOfSgpio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of SGPIO_3"]
    #[inline(always)]
    pub fn intstatus_of_sgpio3(&self) -> IntstatusOfSgpio3R {
        IntstatusOfSgpio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of SGPIO_4"]
    #[inline(always)]
    pub fn intstatus_of_sgpio4(&self) -> IntstatusOfSgpio4R {
        IntstatusOfSgpio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of SGPIO_5"]
    #[inline(always)]
    pub fn intstatus_of_sgpio5(&self) -> IntstatusOfSgpio5R {
        IntstatusOfSgpio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of SGPIO_6"]
    #[inline(always)]
    pub fn intstatus_of_sgpio6(&self) -> IntstatusOfSgpio6R {
        IntstatusOfSgpio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of SGPIO_7"]
    #[inline(always)]
    pub fn intstatus_of_sgpio7(&self) -> IntstatusOfSgpio7R {
        IntstatusOfSgpio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of SGPIO_8"]
    #[inline(always)]
    pub fn intstatus_of_sgpio8(&self) -> IntstatusOfSgpio8R {
        IntstatusOfSgpio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of SGPIO_9"]
    #[inline(always)]
    pub fn intstatus_of_sgpio9(&self) -> IntstatusOfSgpio9R {
        IntstatusOfSgpio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of SGPIO_10"]
    #[inline(always)]
    pub fn intstatus_of_sgpio10(&self) -> IntstatusOfSgpio10R {
        IntstatusOfSgpio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of SGPIO_11"]
    #[inline(always)]
    pub fn intstatus_of_sgpio11(&self) -> IntstatusOfSgpio11R {
        IntstatusOfSgpio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of SGPIO_12"]
    #[inline(always)]
    pub fn intstatus_of_sgpio12(&self) -> IntstatusOfSgpio12R {
        IntstatusOfSgpio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of SGPIO_13"]
    #[inline(always)]
    pub fn intstatus_of_sgpio13(&self) -> IntstatusOfSgpio13R {
        IntstatusOfSgpio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of SGPIO_14"]
    #[inline(always)]
    pub fn intstatus_of_sgpio14(&self) -> IntstatusOfSgpio14R {
        IntstatusOfSgpio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of SGPIO_15"]
    #[inline(always)]
    pub fn intstatus_of_sgpio15(&self) -> IntstatusOfSgpio15R {
        IntstatusOfSgpio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of SGPIO_16"]
    #[inline(always)]
    pub fn intstatus_of_sgpio16(&self) -> IntstatusOfSgpio16R {
        IntstatusOfSgpio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of SGPIO_17"]
    #[inline(always)]
    pub fn intstatus_of_sgpio17(&self) -> IntstatusOfSgpio17R {
        IntstatusOfSgpio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of SGPIO_18"]
    #[inline(always)]
    pub fn intstatus_of_sgpio18(&self) -> IntstatusOfSgpio18R {
        IntstatusOfSgpio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of SGPIO_19"]
    #[inline(always)]
    pub fn intstatus_of_sgpio19(&self) -> IntstatusOfSgpio19R {
        IntstatusOfSgpio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of SGPIO_20"]
    #[inline(always)]
    pub fn intstatus_of_sgpio20(&self) -> IntstatusOfSgpio20R {
        IntstatusOfSgpio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of SGPIO_21"]
    #[inline(always)]
    pub fn intstatus_of_sgpio21(&self) -> IntstatusOfSgpio21R {
        IntstatusOfSgpio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of SGPIO_22"]
    #[inline(always)]
    pub fn intstatus_of_sgpio22(&self) -> IntstatusOfSgpio22R {
        IntstatusOfSgpio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of SGPIO_23"]
    #[inline(always)]
    pub fn intstatus_of_sgpio23(&self) -> IntstatusOfSgpio23R {
        IntstatusOfSgpio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of SGPIO_24"]
    #[inline(always)]
    pub fn intstatus_of_sgpio24(&self) -> IntstatusOfSgpio24R {
        IntstatusOfSgpio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of SGPIO_25"]
    #[inline(always)]
    pub fn intstatus_of_sgpio25(&self) -> IntstatusOfSgpio25R {
        IntstatusOfSgpio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of SGPIO_26"]
    #[inline(always)]
    pub fn intstatus_of_sgpio26(&self) -> IntstatusOfSgpio26R {
        IntstatusOfSgpio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of SGPIO_27"]
    #[inline(always)]
    pub fn intstatus_of_sgpio27(&self) -> IntstatusOfSgpio27R {
        IntstatusOfSgpio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of SGPIO_28"]
    #[inline(always)]
    pub fn intstatus_of_sgpio28(&self) -> IntstatusOfSgpio28R {
        IntstatusOfSgpio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of SGPIO_29"]
    #[inline(always)]
    pub fn intstatus_of_sgpio29(&self) -> IntstatusOfSgpio29R {
        IntstatusOfSgpio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of SGPIO_30"]
    #[inline(always)]
    pub fn intstatus_of_sgpio30(&self) -> IntstatusOfSgpio30R {
        IntstatusOfSgpio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of SGPIO_31"]
    #[inline(always)]
    pub fn intstatus_of_sgpio31(&self) -> IntstatusOfSgpio31R {
        IntstatusOfSgpio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#0\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio40Spec;
impl crate::RegisterSpec for Sgpio40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio40::R`](R) reader structure"]
impl crate::Readable for Sgpio40Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio40::W`](W) writer structure"]
impl crate::Writable for Sgpio40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO40 to value 0"]
impl crate::Resettable for Sgpio40Spec {}
