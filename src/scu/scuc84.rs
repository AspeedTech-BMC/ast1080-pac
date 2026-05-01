#[doc = "Register `SCUC84` reader"]
pub type R = crate::R<Scuc84Spec>;
#[doc = "Register `SCUC84` writer"]
pub type W = crate::W<Scuc84Spec>;
#[doc = "Field `SCUREGSEC29AC` reader - SCU_REG_SEC2_9AC"]
pub type Scuregsec29acR = crate::BitReader;
#[doc = "Field `SCUREGSEC29AC` writer - SCU_REG_SEC2_9AC"]
pub type Scuregsec29acW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC20B0` reader - SCU_REG_SEC2_0B0"]
pub type Scuregsec20b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC20B0` writer - SCU_REG_SEC2_0B0"]
pub type Scuregsec20b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGSEC20BC` reader - SCU_REG_SEC2_0BC"]
pub type Scuregsec20bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC20BC` writer - SCU_REG_SEC2_0BC"]
pub type Scuregsec20bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC20C0` reader - SCU_REG_SEC2_0C0"]
pub type Scuregsec20c0R = crate::BitReader;
#[doc = "Field `SCUREGSEC20C0` writer - SCU_REG_SEC2_0C0"]
pub type Scuregsec20c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC20C4` reader - SCU_REG_SEC2_0C4"]
pub type Scuregsec20c4R = crate::BitReader;
#[doc = "Field `SCUREGSEC20C4` writer - SCU_REG_SEC2_0C4"]
pub type Scuregsec20c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC20C8` reader - SCU_REG_SEC2_0C8"]
pub type Scuregsec20c8R = crate::BitReader;
#[doc = "Field `SCUREGSEC20C8` writer - SCU_REG_SEC2_0C8"]
pub type Scuregsec20c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC20CC` reader - SCU_REG_SEC2_0CC"]
pub type Scuregsec20ccR = crate::BitReader;
#[doc = "Field `SCUREGSEC20CC` writer - SCU_REG_SEC2_0CC"]
pub type Scuregsec20ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC20D0` reader - SCU_REG_SEC2_0D0"]
pub type Scuregsec20d0R = crate::BitReader;
#[doc = "Field `SCUREGSEC20D0` writer - SCU_REG_SEC2_0D0"]
pub type Scuregsec20d0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC20D4` reader - SCU_REG_SEC2_0D4"]
pub type Scuregsec20d4R = crate::BitReader;
#[doc = "Field `SCUREGSEC20D4` writer - SCU_REG_SEC2_0D4"]
pub type Scuregsec20d4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC20E0` reader - SCU_REG_SEC2_0E0"]
pub type Scuregsec20e0R = crate::BitReader;
#[doc = "Field `SCUREGSEC20E0` writer - SCU_REG_SEC2_0E0"]
pub type Scuregsec20e0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC20F0` reader - SCU_REG_SEC2_0F0"]
pub type Scuregsec20f0R = crate::BitReader;
#[doc = "Field `SCUREGSEC20F0` writer - SCU_REG_SEC2_0F0"]
pub type Scuregsec20f0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - SCU_REG_SEC2_9AC"]
    #[inline(always)]
    pub fn scuregsec29ac(&self) -> Scuregsec29acR {
        Scuregsec29acR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCU_REG_SEC2_0B0"]
    #[inline(always)]
    pub fn scuregsec20b0(&self) -> Scuregsec20b0R {
        Scuregsec20b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_REG_SEC2_0BC"]
    #[inline(always)]
    pub fn scuregsec20bc(&self) -> Scuregsec20bcR {
        Scuregsec20bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_REG_SEC2_0C0"]
    #[inline(always)]
    pub fn scuregsec20c0(&self) -> Scuregsec20c0R {
        Scuregsec20c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC2_0C4"]
    #[inline(always)]
    pub fn scuregsec20c4(&self) -> Scuregsec20c4R {
        Scuregsec20c4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_SEC2_0C8"]
    #[inline(always)]
    pub fn scuregsec20c8(&self) -> Scuregsec20c8R {
        Scuregsec20c8R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_SEC2_0CC"]
    #[inline(always)]
    pub fn scuregsec20cc(&self) -> Scuregsec20ccR {
        Scuregsec20ccR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_REG_SEC2_0D0"]
    #[inline(always)]
    pub fn scuregsec20d0(&self) -> Scuregsec20d0R {
        Scuregsec20d0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_SEC2_0D4"]
    #[inline(always)]
    pub fn scuregsec20d4(&self) -> Scuregsec20d4R {
        Scuregsec20d4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_REG_SEC2_0E0"]
    #[inline(always)]
    pub fn scuregsec20e0(&self) -> Scuregsec20e0R {
        Scuregsec20e0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - SCU_REG_SEC2_0F0"]
    #[inline(always)]
    pub fn scuregsec20f0(&self) -> Scuregsec20f0R {
        Scuregsec20f0R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SCU_REG_SEC2_9AC"]
    #[inline(always)]
    pub fn scuregsec29ac(&mut self) -> Scuregsec29acW<Scuc84Spec> {
        Scuregsec29acW::new(self, 11)
    }
    #[doc = "Bit 12 - SCU_REG_SEC2_0B0"]
    #[inline(always)]
    pub fn scuregsec20b0(&mut self) -> Scuregsec20b0W<Scuc84Spec> {
        Scuregsec20b0W::new(self, 12)
    }
    #[doc = "Bit 15 - SCU_REG_SEC2_0BC"]
    #[inline(always)]
    pub fn scuregsec20bc(&mut self) -> Scuregsec20bcW<Scuc84Spec> {
        Scuregsec20bcW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_REG_SEC2_0C0"]
    #[inline(always)]
    pub fn scuregsec20c0(&mut self) -> Scuregsec20c0W<Scuc84Spec> {
        Scuregsec20c0W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC2_0C4"]
    #[inline(always)]
    pub fn scuregsec20c4(&mut self) -> Scuregsec20c4W<Scuc84Spec> {
        Scuregsec20c4W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_SEC2_0C8"]
    #[inline(always)]
    pub fn scuregsec20c8(&mut self) -> Scuregsec20c8W<Scuc84Spec> {
        Scuregsec20c8W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_SEC2_0CC"]
    #[inline(always)]
    pub fn scuregsec20cc(&mut self) -> Scuregsec20ccW<Scuc84Spec> {
        Scuregsec20ccW::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_REG_SEC2_0D0"]
    #[inline(always)]
    pub fn scuregsec20d0(&mut self) -> Scuregsec20d0W<Scuc84Spec> {
        Scuregsec20d0W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_SEC2_0D4"]
    #[inline(always)]
    pub fn scuregsec20d4(&mut self) -> Scuregsec20d4W<Scuc84Spec> {
        Scuregsec20d4W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_REG_SEC2_0E0"]
    #[inline(always)]
    pub fn scuregsec20e0(&mut self) -> Scuregsec20e0W<Scuc84Spec> {
        Scuregsec20e0W::new(self, 22)
    }
    #[doc = "Bit 28 - SCU_REG_SEC2_0F0"]
    #[inline(always)]
    pub fn scuregsec20f0(&mut self) -> Scuregsec20f0W<Scuc84Spec> {
        Scuregsec20f0W::new(self, 28)
    }
}
#[doc = "Secure2 Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc84::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc84::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc84Spec;
impl crate::RegisterSpec for Scuc84Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc84::R`](R) reader structure"]
impl crate::Readable for Scuc84Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc84::W`](W) writer structure"]
impl crate::Writable for Scuc84Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC84 to value 0"]
impl crate::Resettable for Scuc84Spec {}
