#[doc = "Register `GPIO104` reader"]
pub type R = crate::R<Gpio104Spec>;
#[doc = "Register `GPIO104` writer"]
pub type W = crate::W<Gpio104Spec>;
#[doc = "Field `INTStatusOfGPIO032` reader - Interrupt Status of GPIO032"]
pub type IntstatusOfGpio032R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO033` reader - Interrupt Status of GPIO033"]
pub type IntstatusOfGpio033R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO034` reader - Interrupt Status of GPIO034"]
pub type IntstatusOfGpio034R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO035` reader - Interrupt Status of GPIO035"]
pub type IntstatusOfGpio035R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO036` reader - Interrupt Status of GPIO036"]
pub type IntstatusOfGpio036R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO037` reader - Interrupt Status of GPIO037"]
pub type IntstatusOfGpio037R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO038` reader - Interrupt Status of GPIO038"]
pub type IntstatusOfGpio038R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO039` reader - Interrupt Status of GPIO039"]
pub type IntstatusOfGpio039R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO040` reader - Interrupt Status of GPIO040"]
pub type IntstatusOfGpio040R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO041` reader - Interrupt Status of GPIO041"]
pub type IntstatusOfGpio041R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO042` reader - Interrupt Status of GPIO042"]
pub type IntstatusOfGpio042R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO043` reader - Interrupt Status of GPIO043"]
pub type IntstatusOfGpio043R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO044` reader - Interrupt Status of GPIO044"]
pub type IntstatusOfGpio044R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO045` reader - Interrupt Status of GPIO045"]
pub type IntstatusOfGpio045R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO046` reader - Interrupt Status of GPIO046"]
pub type IntstatusOfGpio046R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO047` reader - Interrupt Status of GPIO047"]
pub type IntstatusOfGpio047R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO048` reader - Interrupt Status of GPIO048"]
pub type IntstatusOfGpio048R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO049` reader - Interrupt Status of GPIO049"]
pub type IntstatusOfGpio049R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO050` reader - Interrupt Status of GPIO050"]
pub type IntstatusOfGpio050R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO051` reader - Interrupt Status of GPIO051"]
pub type IntstatusOfGpio051R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO052` reader - Interrupt Status of GPIO052"]
pub type IntstatusOfGpio052R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO053` reader - Interrupt Status of GPIO053"]
pub type IntstatusOfGpio053R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO054` reader - Interrupt Status of GPIO054"]
pub type IntstatusOfGpio054R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO055` reader - Interrupt Status of GPIO055"]
pub type IntstatusOfGpio055R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO056` reader - Interrupt Status of GPIO056"]
pub type IntstatusOfGpio056R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO057` reader - Interrupt Status of GPIO057"]
pub type IntstatusOfGpio057R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO058` reader - Interrupt Status of GPIO058"]
pub type IntstatusOfGpio058R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO059` reader - Interrupt Status of GPIO059"]
pub type IntstatusOfGpio059R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO060` reader - Interrupt Status of GPIO060"]
pub type IntstatusOfGpio060R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO061` reader - Interrupt Status of GPIO061"]
pub type IntstatusOfGpio061R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO062` reader - Interrupt Status of GPIO062"]
pub type IntstatusOfGpio062R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO063` reader - Interrupt Status of GPIO063"]
pub type IntstatusOfGpio063R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of GPIO032"]
    #[inline(always)]
    pub fn intstatus_of_gpio032(&self) -> IntstatusOfGpio032R {
        IntstatusOfGpio032R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of GPIO033"]
    #[inline(always)]
    pub fn intstatus_of_gpio033(&self) -> IntstatusOfGpio033R {
        IntstatusOfGpio033R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of GPIO034"]
    #[inline(always)]
    pub fn intstatus_of_gpio034(&self) -> IntstatusOfGpio034R {
        IntstatusOfGpio034R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of GPIO035"]
    #[inline(always)]
    pub fn intstatus_of_gpio035(&self) -> IntstatusOfGpio035R {
        IntstatusOfGpio035R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of GPIO036"]
    #[inline(always)]
    pub fn intstatus_of_gpio036(&self) -> IntstatusOfGpio036R {
        IntstatusOfGpio036R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of GPIO037"]
    #[inline(always)]
    pub fn intstatus_of_gpio037(&self) -> IntstatusOfGpio037R {
        IntstatusOfGpio037R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of GPIO038"]
    #[inline(always)]
    pub fn intstatus_of_gpio038(&self) -> IntstatusOfGpio038R {
        IntstatusOfGpio038R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of GPIO039"]
    #[inline(always)]
    pub fn intstatus_of_gpio039(&self) -> IntstatusOfGpio039R {
        IntstatusOfGpio039R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of GPIO040"]
    #[inline(always)]
    pub fn intstatus_of_gpio040(&self) -> IntstatusOfGpio040R {
        IntstatusOfGpio040R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of GPIO041"]
    #[inline(always)]
    pub fn intstatus_of_gpio041(&self) -> IntstatusOfGpio041R {
        IntstatusOfGpio041R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of GPIO042"]
    #[inline(always)]
    pub fn intstatus_of_gpio042(&self) -> IntstatusOfGpio042R {
        IntstatusOfGpio042R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of GPIO043"]
    #[inline(always)]
    pub fn intstatus_of_gpio043(&self) -> IntstatusOfGpio043R {
        IntstatusOfGpio043R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of GPIO044"]
    #[inline(always)]
    pub fn intstatus_of_gpio044(&self) -> IntstatusOfGpio044R {
        IntstatusOfGpio044R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of GPIO045"]
    #[inline(always)]
    pub fn intstatus_of_gpio045(&self) -> IntstatusOfGpio045R {
        IntstatusOfGpio045R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of GPIO046"]
    #[inline(always)]
    pub fn intstatus_of_gpio046(&self) -> IntstatusOfGpio046R {
        IntstatusOfGpio046R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of GPIO047"]
    #[inline(always)]
    pub fn intstatus_of_gpio047(&self) -> IntstatusOfGpio047R {
        IntstatusOfGpio047R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of GPIO048"]
    #[inline(always)]
    pub fn intstatus_of_gpio048(&self) -> IntstatusOfGpio048R {
        IntstatusOfGpio048R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of GPIO049"]
    #[inline(always)]
    pub fn intstatus_of_gpio049(&self) -> IntstatusOfGpio049R {
        IntstatusOfGpio049R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of GPIO050"]
    #[inline(always)]
    pub fn intstatus_of_gpio050(&self) -> IntstatusOfGpio050R {
        IntstatusOfGpio050R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of GPIO051"]
    #[inline(always)]
    pub fn intstatus_of_gpio051(&self) -> IntstatusOfGpio051R {
        IntstatusOfGpio051R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of GPIO052"]
    #[inline(always)]
    pub fn intstatus_of_gpio052(&self) -> IntstatusOfGpio052R {
        IntstatusOfGpio052R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of GPIO053"]
    #[inline(always)]
    pub fn intstatus_of_gpio053(&self) -> IntstatusOfGpio053R {
        IntstatusOfGpio053R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of GPIO054"]
    #[inline(always)]
    pub fn intstatus_of_gpio054(&self) -> IntstatusOfGpio054R {
        IntstatusOfGpio054R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of GPIO055"]
    #[inline(always)]
    pub fn intstatus_of_gpio055(&self) -> IntstatusOfGpio055R {
        IntstatusOfGpio055R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of GPIO056"]
    #[inline(always)]
    pub fn intstatus_of_gpio056(&self) -> IntstatusOfGpio056R {
        IntstatusOfGpio056R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of GPIO057"]
    #[inline(always)]
    pub fn intstatus_of_gpio057(&self) -> IntstatusOfGpio057R {
        IntstatusOfGpio057R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of GPIO058"]
    #[inline(always)]
    pub fn intstatus_of_gpio058(&self) -> IntstatusOfGpio058R {
        IntstatusOfGpio058R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of GPIO059"]
    #[inline(always)]
    pub fn intstatus_of_gpio059(&self) -> IntstatusOfGpio059R {
        IntstatusOfGpio059R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of GPIO060"]
    #[inline(always)]
    pub fn intstatus_of_gpio060(&self) -> IntstatusOfGpio060R {
        IntstatusOfGpio060R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of GPIO061"]
    #[inline(always)]
    pub fn intstatus_of_gpio061(&self) -> IntstatusOfGpio061R {
        IntstatusOfGpio061R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of GPIO062"]
    #[inline(always)]
    pub fn intstatus_of_gpio062(&self) -> IntstatusOfGpio062R {
        IntstatusOfGpio062R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of GPIO063"]
    #[inline(always)]
    pub fn intstatus_of_gpio063(&self) -> IntstatusOfGpio063R {
        IntstatusOfGpio063R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio104Spec;
impl crate::RegisterSpec for Gpio104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio104::R`](R) reader structure"]
impl crate::Readable for Gpio104Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio104::W`](W) writer structure"]
impl crate::Writable for Gpio104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO104 to value 0"]
impl crate::Resettable for Gpio104Spec {}
