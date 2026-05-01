#[doc = "Register `SGPIO044` reader"]
pub type R = crate::R<Sgpio044Spec>;
#[doc = "Register `SGPIO044` writer"]
pub type W = crate::W<Sgpio044Spec>;
#[doc = "Field `INTStatusOfSGPIO32` reader - Interrupt Status of SGPIO_32"]
pub type IntstatusOfSgpio32R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO33` reader - Interrupt Status of SGPIO_33"]
pub type IntstatusOfSgpio33R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO34` reader - Interrupt Status of SGPIO_34"]
pub type IntstatusOfSgpio34R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO35` reader - Interrupt Status of SGPIO_35"]
pub type IntstatusOfSgpio35R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO36` reader - Interrupt Status of SGPIO_36"]
pub type IntstatusOfSgpio36R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO37` reader - Interrupt Status of SGPIO_37"]
pub type IntstatusOfSgpio37R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO38` reader - Interrupt Status of SGPIO_38"]
pub type IntstatusOfSgpio38R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO39` reader - Interrupt Status of SGPIO_39"]
pub type IntstatusOfSgpio39R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO40` reader - Interrupt Status of SGPIO_40"]
pub type IntstatusOfSgpio40R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO41` reader - Interrupt Status of SGPIO_41"]
pub type IntstatusOfSgpio41R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO42` reader - Interrupt Status of SGPIO_42"]
pub type IntstatusOfSgpio42R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO43` reader - Interrupt Status of SGPIO_43"]
pub type IntstatusOfSgpio43R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO44` reader - Interrupt Status of SGPIO_44"]
pub type IntstatusOfSgpio44R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO45` reader - Interrupt Status of SGPIO_45"]
pub type IntstatusOfSgpio45R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO46` reader - Interrupt Status of SGPIO_46"]
pub type IntstatusOfSgpio46R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO47` reader - Interrupt Status of SGPIO_47"]
pub type IntstatusOfSgpio47R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO48` reader - Interrupt Status of SGPIO_48"]
pub type IntstatusOfSgpio48R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO49` reader - Interrupt Status of SGPIO_49"]
pub type IntstatusOfSgpio49R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO50` reader - Interrupt Status of SGPIO_50"]
pub type IntstatusOfSgpio50R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO51` reader - Interrupt Status of SGPIO_51"]
pub type IntstatusOfSgpio51R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO52` reader - Interrupt Status of SGPIO_52"]
pub type IntstatusOfSgpio52R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO53` reader - Interrupt Status of SGPIO_53"]
pub type IntstatusOfSgpio53R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO54` reader - Interrupt Status of SGPIO_54"]
pub type IntstatusOfSgpio54R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO55` reader - Interrupt Status of SGPIO_55"]
pub type IntstatusOfSgpio55R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO56` reader - Interrupt Status of SGPIO_56"]
pub type IntstatusOfSgpio56R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO57` reader - Interrupt Status of SGPIO_57"]
pub type IntstatusOfSgpio57R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO58` reader - Interrupt Status of SGPIO_58"]
pub type IntstatusOfSgpio58R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO59` reader - Interrupt Status of SGPIO_59"]
pub type IntstatusOfSgpio59R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO60` reader - Interrupt Status of SGPIO_60"]
pub type IntstatusOfSgpio60R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO61` reader - Interrupt Status of SGPIO_61"]
pub type IntstatusOfSgpio61R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO62` reader - Interrupt Status of SGPIO_62"]
pub type IntstatusOfSgpio62R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO63` reader - Interrupt Status of SGPIO_63"]
pub type IntstatusOfSgpio63R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of SGPIO_32"]
    #[inline(always)]
    pub fn intstatus_of_sgpio32(&self) -> IntstatusOfSgpio32R {
        IntstatusOfSgpio32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of SGPIO_33"]
    #[inline(always)]
    pub fn intstatus_of_sgpio33(&self) -> IntstatusOfSgpio33R {
        IntstatusOfSgpio33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of SGPIO_34"]
    #[inline(always)]
    pub fn intstatus_of_sgpio34(&self) -> IntstatusOfSgpio34R {
        IntstatusOfSgpio34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of SGPIO_35"]
    #[inline(always)]
    pub fn intstatus_of_sgpio35(&self) -> IntstatusOfSgpio35R {
        IntstatusOfSgpio35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of SGPIO_36"]
    #[inline(always)]
    pub fn intstatus_of_sgpio36(&self) -> IntstatusOfSgpio36R {
        IntstatusOfSgpio36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of SGPIO_37"]
    #[inline(always)]
    pub fn intstatus_of_sgpio37(&self) -> IntstatusOfSgpio37R {
        IntstatusOfSgpio37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of SGPIO_38"]
    #[inline(always)]
    pub fn intstatus_of_sgpio38(&self) -> IntstatusOfSgpio38R {
        IntstatusOfSgpio38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of SGPIO_39"]
    #[inline(always)]
    pub fn intstatus_of_sgpio39(&self) -> IntstatusOfSgpio39R {
        IntstatusOfSgpio39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of SGPIO_40"]
    #[inline(always)]
    pub fn intstatus_of_sgpio40(&self) -> IntstatusOfSgpio40R {
        IntstatusOfSgpio40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of SGPIO_41"]
    #[inline(always)]
    pub fn intstatus_of_sgpio41(&self) -> IntstatusOfSgpio41R {
        IntstatusOfSgpio41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of SGPIO_42"]
    #[inline(always)]
    pub fn intstatus_of_sgpio42(&self) -> IntstatusOfSgpio42R {
        IntstatusOfSgpio42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of SGPIO_43"]
    #[inline(always)]
    pub fn intstatus_of_sgpio43(&self) -> IntstatusOfSgpio43R {
        IntstatusOfSgpio43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of SGPIO_44"]
    #[inline(always)]
    pub fn intstatus_of_sgpio44(&self) -> IntstatusOfSgpio44R {
        IntstatusOfSgpio44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of SGPIO_45"]
    #[inline(always)]
    pub fn intstatus_of_sgpio45(&self) -> IntstatusOfSgpio45R {
        IntstatusOfSgpio45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of SGPIO_46"]
    #[inline(always)]
    pub fn intstatus_of_sgpio46(&self) -> IntstatusOfSgpio46R {
        IntstatusOfSgpio46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of SGPIO_47"]
    #[inline(always)]
    pub fn intstatus_of_sgpio47(&self) -> IntstatusOfSgpio47R {
        IntstatusOfSgpio47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of SGPIO_48"]
    #[inline(always)]
    pub fn intstatus_of_sgpio48(&self) -> IntstatusOfSgpio48R {
        IntstatusOfSgpio48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of SGPIO_49"]
    #[inline(always)]
    pub fn intstatus_of_sgpio49(&self) -> IntstatusOfSgpio49R {
        IntstatusOfSgpio49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of SGPIO_50"]
    #[inline(always)]
    pub fn intstatus_of_sgpio50(&self) -> IntstatusOfSgpio50R {
        IntstatusOfSgpio50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of SGPIO_51"]
    #[inline(always)]
    pub fn intstatus_of_sgpio51(&self) -> IntstatusOfSgpio51R {
        IntstatusOfSgpio51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of SGPIO_52"]
    #[inline(always)]
    pub fn intstatus_of_sgpio52(&self) -> IntstatusOfSgpio52R {
        IntstatusOfSgpio52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of SGPIO_53"]
    #[inline(always)]
    pub fn intstatus_of_sgpio53(&self) -> IntstatusOfSgpio53R {
        IntstatusOfSgpio53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of SGPIO_54"]
    #[inline(always)]
    pub fn intstatus_of_sgpio54(&self) -> IntstatusOfSgpio54R {
        IntstatusOfSgpio54R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of SGPIO_55"]
    #[inline(always)]
    pub fn intstatus_of_sgpio55(&self) -> IntstatusOfSgpio55R {
        IntstatusOfSgpio55R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of SGPIO_56"]
    #[inline(always)]
    pub fn intstatus_of_sgpio56(&self) -> IntstatusOfSgpio56R {
        IntstatusOfSgpio56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of SGPIO_57"]
    #[inline(always)]
    pub fn intstatus_of_sgpio57(&self) -> IntstatusOfSgpio57R {
        IntstatusOfSgpio57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of SGPIO_58"]
    #[inline(always)]
    pub fn intstatus_of_sgpio58(&self) -> IntstatusOfSgpio58R {
        IntstatusOfSgpio58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of SGPIO_59"]
    #[inline(always)]
    pub fn intstatus_of_sgpio59(&self) -> IntstatusOfSgpio59R {
        IntstatusOfSgpio59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of SGPIO_60"]
    #[inline(always)]
    pub fn intstatus_of_sgpio60(&self) -> IntstatusOfSgpio60R {
        IntstatusOfSgpio60R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of SGPIO_61"]
    #[inline(always)]
    pub fn intstatus_of_sgpio61(&self) -> IntstatusOfSgpio61R {
        IntstatusOfSgpio61R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of SGPIO_62"]
    #[inline(always)]
    pub fn intstatus_of_sgpio62(&self) -> IntstatusOfSgpio62R {
        IntstatusOfSgpio62R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of SGPIO_63"]
    #[inline(always)]
    pub fn intstatus_of_sgpio63(&self) -> IntstatusOfSgpio63R {
        IntstatusOfSgpio63R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio044Spec;
impl crate::RegisterSpec for Sgpio044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio044::R`](R) reader structure"]
impl crate::Readable for Sgpio044Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio044::W`](W) writer structure"]
impl crate::Writable for Sgpio044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO044 to value 0"]
impl crate::Resettable for Sgpio044Spec {}
