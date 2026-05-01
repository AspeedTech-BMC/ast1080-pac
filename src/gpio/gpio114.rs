#[doc = "Register `GPIO114` reader"]
pub type R = crate::R<Gpio114Spec>;
#[doc = "Register `GPIO114` writer"]
pub type W = crate::W<Gpio114Spec>;
#[doc = "Field `INTStatusOfGPIO160` reader - Interrupt Status of GPIO160"]
pub type IntstatusOfGpio160R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO161` reader - Interrupt Status of GPIO161"]
pub type IntstatusOfGpio161R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO162` reader - Interrupt Status of GPIO162"]
pub type IntstatusOfGpio162R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO163` reader - Interrupt Status of GPIO163"]
pub type IntstatusOfGpio163R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO164` reader - Interrupt Status of GPIO164"]
pub type IntstatusOfGpio164R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO165` reader - Interrupt Status of GPIO165"]
pub type IntstatusOfGpio165R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO166` reader - Interrupt Status of GPIO166"]
pub type IntstatusOfGpio166R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO167` reader - Interrupt Status of GPIO167"]
pub type IntstatusOfGpio167R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO168` reader - Interrupt Status of GPIO168"]
pub type IntstatusOfGpio168R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO169` reader - Interrupt Status of GPIO169"]
pub type IntstatusOfGpio169R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO170` reader - Interrupt Status of GPIO170"]
pub type IntstatusOfGpio170R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO171` reader - Interrupt Status of GPIO171"]
pub type IntstatusOfGpio171R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO172` reader - Interrupt Status of GPIO172"]
pub type IntstatusOfGpio172R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO173` reader - Interrupt Status of GPIO173"]
pub type IntstatusOfGpio173R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO174` reader - Interrupt Status of GPIO174"]
pub type IntstatusOfGpio174R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO175` reader - Interrupt Status of GPIO175"]
pub type IntstatusOfGpio175R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO176` reader - Interrupt Status of GPIO176"]
pub type IntstatusOfGpio176R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO177` reader - Interrupt Status of GPIO177"]
pub type IntstatusOfGpio177R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO178` reader - Interrupt Status of GPIO178"]
pub type IntstatusOfGpio178R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO179` reader - Interrupt Status of GPIO179"]
pub type IntstatusOfGpio179R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO180` reader - Interrupt Status of GPIO180"]
pub type IntstatusOfGpio180R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO181` reader - Interrupt Status of GPIO181"]
pub type IntstatusOfGpio181R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO182` reader - Interrupt Status of GPIO182"]
pub type IntstatusOfGpio182R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO183` reader - Interrupt Status of GPIO183"]
pub type IntstatusOfGpio183R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO184` reader - Interrupt Status of GPIO184"]
pub type IntstatusOfGpio184R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO185` reader - Interrupt Status of GPIO185"]
pub type IntstatusOfGpio185R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO186` reader - Interrupt Status of GPIO186"]
pub type IntstatusOfGpio186R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO187` reader - Interrupt Status of GPIO187"]
pub type IntstatusOfGpio187R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO188` reader - Interrupt Status of GPIO188"]
pub type IntstatusOfGpio188R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO189` reader - Interrupt Status of GPIO189"]
pub type IntstatusOfGpio189R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO190` reader - Interrupt Status of GPIO190"]
pub type IntstatusOfGpio190R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO191` reader - Interrupt Status of GPIO191"]
pub type IntstatusOfGpio191R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of GPIO160"]
    #[inline(always)]
    pub fn intstatus_of_gpio160(&self) -> IntstatusOfGpio160R {
        IntstatusOfGpio160R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of GPIO161"]
    #[inline(always)]
    pub fn intstatus_of_gpio161(&self) -> IntstatusOfGpio161R {
        IntstatusOfGpio161R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of GPIO162"]
    #[inline(always)]
    pub fn intstatus_of_gpio162(&self) -> IntstatusOfGpio162R {
        IntstatusOfGpio162R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of GPIO163"]
    #[inline(always)]
    pub fn intstatus_of_gpio163(&self) -> IntstatusOfGpio163R {
        IntstatusOfGpio163R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of GPIO164"]
    #[inline(always)]
    pub fn intstatus_of_gpio164(&self) -> IntstatusOfGpio164R {
        IntstatusOfGpio164R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of GPIO165"]
    #[inline(always)]
    pub fn intstatus_of_gpio165(&self) -> IntstatusOfGpio165R {
        IntstatusOfGpio165R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of GPIO166"]
    #[inline(always)]
    pub fn intstatus_of_gpio166(&self) -> IntstatusOfGpio166R {
        IntstatusOfGpio166R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of GPIO167"]
    #[inline(always)]
    pub fn intstatus_of_gpio167(&self) -> IntstatusOfGpio167R {
        IntstatusOfGpio167R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of GPIO168"]
    #[inline(always)]
    pub fn intstatus_of_gpio168(&self) -> IntstatusOfGpio168R {
        IntstatusOfGpio168R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of GPIO169"]
    #[inline(always)]
    pub fn intstatus_of_gpio169(&self) -> IntstatusOfGpio169R {
        IntstatusOfGpio169R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of GPIO170"]
    #[inline(always)]
    pub fn intstatus_of_gpio170(&self) -> IntstatusOfGpio170R {
        IntstatusOfGpio170R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of GPIO171"]
    #[inline(always)]
    pub fn intstatus_of_gpio171(&self) -> IntstatusOfGpio171R {
        IntstatusOfGpio171R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of GPIO172"]
    #[inline(always)]
    pub fn intstatus_of_gpio172(&self) -> IntstatusOfGpio172R {
        IntstatusOfGpio172R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of GPIO173"]
    #[inline(always)]
    pub fn intstatus_of_gpio173(&self) -> IntstatusOfGpio173R {
        IntstatusOfGpio173R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of GPIO174"]
    #[inline(always)]
    pub fn intstatus_of_gpio174(&self) -> IntstatusOfGpio174R {
        IntstatusOfGpio174R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of GPIO175"]
    #[inline(always)]
    pub fn intstatus_of_gpio175(&self) -> IntstatusOfGpio175R {
        IntstatusOfGpio175R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of GPIO176"]
    #[inline(always)]
    pub fn intstatus_of_gpio176(&self) -> IntstatusOfGpio176R {
        IntstatusOfGpio176R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of GPIO177"]
    #[inline(always)]
    pub fn intstatus_of_gpio177(&self) -> IntstatusOfGpio177R {
        IntstatusOfGpio177R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of GPIO178"]
    #[inline(always)]
    pub fn intstatus_of_gpio178(&self) -> IntstatusOfGpio178R {
        IntstatusOfGpio178R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of GPIO179"]
    #[inline(always)]
    pub fn intstatus_of_gpio179(&self) -> IntstatusOfGpio179R {
        IntstatusOfGpio179R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of GPIO180"]
    #[inline(always)]
    pub fn intstatus_of_gpio180(&self) -> IntstatusOfGpio180R {
        IntstatusOfGpio180R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of GPIO181"]
    #[inline(always)]
    pub fn intstatus_of_gpio181(&self) -> IntstatusOfGpio181R {
        IntstatusOfGpio181R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of GPIO182"]
    #[inline(always)]
    pub fn intstatus_of_gpio182(&self) -> IntstatusOfGpio182R {
        IntstatusOfGpio182R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of GPIO183"]
    #[inline(always)]
    pub fn intstatus_of_gpio183(&self) -> IntstatusOfGpio183R {
        IntstatusOfGpio183R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of GPIO184"]
    #[inline(always)]
    pub fn intstatus_of_gpio184(&self) -> IntstatusOfGpio184R {
        IntstatusOfGpio184R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of GPIO185"]
    #[inline(always)]
    pub fn intstatus_of_gpio185(&self) -> IntstatusOfGpio185R {
        IntstatusOfGpio185R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of GPIO186"]
    #[inline(always)]
    pub fn intstatus_of_gpio186(&self) -> IntstatusOfGpio186R {
        IntstatusOfGpio186R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of GPIO187"]
    #[inline(always)]
    pub fn intstatus_of_gpio187(&self) -> IntstatusOfGpio187R {
        IntstatusOfGpio187R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of GPIO188"]
    #[inline(always)]
    pub fn intstatus_of_gpio188(&self) -> IntstatusOfGpio188R {
        IntstatusOfGpio188R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of GPIO189"]
    #[inline(always)]
    pub fn intstatus_of_gpio189(&self) -> IntstatusOfGpio189R {
        IntstatusOfGpio189R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of GPIO190"]
    #[inline(always)]
    pub fn intstatus_of_gpio190(&self) -> IntstatusOfGpio190R {
        IntstatusOfGpio190R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of GPIO191"]
    #[inline(always)]
    pub fn intstatus_of_gpio191(&self) -> IntstatusOfGpio191R {
        IntstatusOfGpio191R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio114Spec;
impl crate::RegisterSpec for Gpio114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio114::R`](R) reader structure"]
impl crate::Readable for Gpio114Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio114::W`](W) writer structure"]
impl crate::Writable for Gpio114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO114 to value 0"]
impl crate::Resettable for Gpio114Spec {}
