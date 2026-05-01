#[doc = "Register `GPIO10C` reader"]
pub type R = crate::R<Gpio10cSpec>;
#[doc = "Register `GPIO10C` writer"]
pub type W = crate::W<Gpio10cSpec>;
#[doc = "Field `INTStatusOfGPIO096` reader - Interrupt Status of GPIO096"]
pub type IntstatusOfGpio096R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO097` reader - Interrupt Status of GPIO097"]
pub type IntstatusOfGpio097R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO098` reader - Interrupt Status of GPIO098"]
pub type IntstatusOfGpio098R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO099` reader - Interrupt Status of GPIO099"]
pub type IntstatusOfGpio099R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO100` reader - Interrupt Status of GPIO100"]
pub type IntstatusOfGpio100R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO101` reader - Interrupt Status of GPIO101"]
pub type IntstatusOfGpio101R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO102` reader - Interrupt Status of GPIO102"]
pub type IntstatusOfGpio102R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO103` reader - Interrupt Status of GPIO103"]
pub type IntstatusOfGpio103R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO104` reader - Interrupt Status of GPIO104"]
pub type IntstatusOfGpio104R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO105` reader - Interrupt Status of GPIO105"]
pub type IntstatusOfGpio105R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO106` reader - Interrupt Status of GPIO106"]
pub type IntstatusOfGpio106R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO107` reader - Interrupt Status of GPIO107"]
pub type IntstatusOfGpio107R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO108` reader - Interrupt Status of GPIO108"]
pub type IntstatusOfGpio108R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO109` reader - Interrupt Status of GPIO109"]
pub type IntstatusOfGpio109R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO110` reader - Interrupt Status of GPIO110"]
pub type IntstatusOfGpio110R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO111` reader - Interrupt Status of GPIO111"]
pub type IntstatusOfGpio111R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO112` reader - Interrupt Status of GPIO112"]
pub type IntstatusOfGpio112R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO113` reader - Interrupt Status of GPIO113"]
pub type IntstatusOfGpio113R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO114` reader - Interrupt Status of GPIO114"]
pub type IntstatusOfGpio114R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO115` reader - Interrupt Status of GPIO115"]
pub type IntstatusOfGpio115R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO116` reader - Interrupt Status of GPIO116"]
pub type IntstatusOfGpio116R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO117` reader - Interrupt Status of GPIO117"]
pub type IntstatusOfGpio117R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO118` reader - Interrupt Status of GPIO118"]
pub type IntstatusOfGpio118R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO119` reader - Interrupt Status of GPIO119"]
pub type IntstatusOfGpio119R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO120` reader - Interrupt Status of GPIO120"]
pub type IntstatusOfGpio120R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO121` reader - Interrupt Status of GPIO121"]
pub type IntstatusOfGpio121R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO122` reader - Interrupt Status of GPIO122"]
pub type IntstatusOfGpio122R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO123` reader - Interrupt Status of GPIO123"]
pub type IntstatusOfGpio123R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO124` reader - Interrupt Status of GPIO124"]
pub type IntstatusOfGpio124R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO125` reader - Interrupt Status of GPIO125"]
pub type IntstatusOfGpio125R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO126` reader - Interrupt Status of GPIO126"]
pub type IntstatusOfGpio126R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO127` reader - Interrupt Status of GPIO127"]
pub type IntstatusOfGpio127R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of GPIO096"]
    #[inline(always)]
    pub fn intstatus_of_gpio096(&self) -> IntstatusOfGpio096R {
        IntstatusOfGpio096R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of GPIO097"]
    #[inline(always)]
    pub fn intstatus_of_gpio097(&self) -> IntstatusOfGpio097R {
        IntstatusOfGpio097R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of GPIO098"]
    #[inline(always)]
    pub fn intstatus_of_gpio098(&self) -> IntstatusOfGpio098R {
        IntstatusOfGpio098R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of GPIO099"]
    #[inline(always)]
    pub fn intstatus_of_gpio099(&self) -> IntstatusOfGpio099R {
        IntstatusOfGpio099R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of GPIO100"]
    #[inline(always)]
    pub fn intstatus_of_gpio100(&self) -> IntstatusOfGpio100R {
        IntstatusOfGpio100R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of GPIO101"]
    #[inline(always)]
    pub fn intstatus_of_gpio101(&self) -> IntstatusOfGpio101R {
        IntstatusOfGpio101R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of GPIO102"]
    #[inline(always)]
    pub fn intstatus_of_gpio102(&self) -> IntstatusOfGpio102R {
        IntstatusOfGpio102R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of GPIO103"]
    #[inline(always)]
    pub fn intstatus_of_gpio103(&self) -> IntstatusOfGpio103R {
        IntstatusOfGpio103R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of GPIO104"]
    #[inline(always)]
    pub fn intstatus_of_gpio104(&self) -> IntstatusOfGpio104R {
        IntstatusOfGpio104R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of GPIO105"]
    #[inline(always)]
    pub fn intstatus_of_gpio105(&self) -> IntstatusOfGpio105R {
        IntstatusOfGpio105R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of GPIO106"]
    #[inline(always)]
    pub fn intstatus_of_gpio106(&self) -> IntstatusOfGpio106R {
        IntstatusOfGpio106R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of GPIO107"]
    #[inline(always)]
    pub fn intstatus_of_gpio107(&self) -> IntstatusOfGpio107R {
        IntstatusOfGpio107R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of GPIO108"]
    #[inline(always)]
    pub fn intstatus_of_gpio108(&self) -> IntstatusOfGpio108R {
        IntstatusOfGpio108R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of GPIO109"]
    #[inline(always)]
    pub fn intstatus_of_gpio109(&self) -> IntstatusOfGpio109R {
        IntstatusOfGpio109R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of GPIO110"]
    #[inline(always)]
    pub fn intstatus_of_gpio110(&self) -> IntstatusOfGpio110R {
        IntstatusOfGpio110R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of GPIO111"]
    #[inline(always)]
    pub fn intstatus_of_gpio111(&self) -> IntstatusOfGpio111R {
        IntstatusOfGpio111R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of GPIO112"]
    #[inline(always)]
    pub fn intstatus_of_gpio112(&self) -> IntstatusOfGpio112R {
        IntstatusOfGpio112R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of GPIO113"]
    #[inline(always)]
    pub fn intstatus_of_gpio113(&self) -> IntstatusOfGpio113R {
        IntstatusOfGpio113R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of GPIO114"]
    #[inline(always)]
    pub fn intstatus_of_gpio114(&self) -> IntstatusOfGpio114R {
        IntstatusOfGpio114R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of GPIO115"]
    #[inline(always)]
    pub fn intstatus_of_gpio115(&self) -> IntstatusOfGpio115R {
        IntstatusOfGpio115R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of GPIO116"]
    #[inline(always)]
    pub fn intstatus_of_gpio116(&self) -> IntstatusOfGpio116R {
        IntstatusOfGpio116R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of GPIO117"]
    #[inline(always)]
    pub fn intstatus_of_gpio117(&self) -> IntstatusOfGpio117R {
        IntstatusOfGpio117R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of GPIO118"]
    #[inline(always)]
    pub fn intstatus_of_gpio118(&self) -> IntstatusOfGpio118R {
        IntstatusOfGpio118R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of GPIO119"]
    #[inline(always)]
    pub fn intstatus_of_gpio119(&self) -> IntstatusOfGpio119R {
        IntstatusOfGpio119R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of GPIO120"]
    #[inline(always)]
    pub fn intstatus_of_gpio120(&self) -> IntstatusOfGpio120R {
        IntstatusOfGpio120R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of GPIO121"]
    #[inline(always)]
    pub fn intstatus_of_gpio121(&self) -> IntstatusOfGpio121R {
        IntstatusOfGpio121R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of GPIO122"]
    #[inline(always)]
    pub fn intstatus_of_gpio122(&self) -> IntstatusOfGpio122R {
        IntstatusOfGpio122R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of GPIO123"]
    #[inline(always)]
    pub fn intstatus_of_gpio123(&self) -> IntstatusOfGpio123R {
        IntstatusOfGpio123R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of GPIO124"]
    #[inline(always)]
    pub fn intstatus_of_gpio124(&self) -> IntstatusOfGpio124R {
        IntstatusOfGpio124R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of GPIO125"]
    #[inline(always)]
    pub fn intstatus_of_gpio125(&self) -> IntstatusOfGpio125R {
        IntstatusOfGpio125R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of GPIO126"]
    #[inline(always)]
    pub fn intstatus_of_gpio126(&self) -> IntstatusOfGpio126R {
        IntstatusOfGpio126R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of GPIO127"]
    #[inline(always)]
    pub fn intstatus_of_gpio127(&self) -> IntstatusOfGpio127R {
        IntstatusOfGpio127R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio10c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio10c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio10cSpec;
impl crate::RegisterSpec for Gpio10cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio10c::R`](R) reader structure"]
impl crate::Readable for Gpio10cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio10c::W`](W) writer structure"]
impl crate::Writable for Gpio10cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO10C to value 0"]
impl crate::Resettable for Gpio10cSpec {}
