#[doc = "Register `SCUC04` reader"]
pub type R = crate::R<Scuc04Spec>;
#[doc = "Register `SCUC04` writer"]
pub type W = crate::W<Scuc04Spec>;
#[doc = "Field `SCUREGSEC10AC` reader - SCU_REG_SEC1_0AC"]
pub type Scuregsec10acR = crate::BitReader;
#[doc = "Field `SCUREGSEC10AC` writer - SCU_REG_SEC1_0AC"]
pub type Scuregsec10acW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC10B0` reader - SCU_REG_SEC1_0B0"]
pub type Scuregsec10b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC10B0` writer - SCU_REG_SEC1_0B0"]
pub type Scuregsec10b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGSEC10BC` reader - SCU_REG_SEC1_0BC"]
pub type Scuregsec10bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC10BC` writer - SCU_REG_SEC1_0BC"]
pub type Scuregsec10bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC10C0` reader - SCU_REG_SEC1_0C0"]
pub type Scuregsec10c0R = crate::BitReader;
#[doc = "Field `SCUREGSEC10C0` writer - SCU_REG_SEC1_0C0"]
pub type Scuregsec10c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC10C4` reader - SCU_REG_SEC1_0C4"]
pub type Scuregsec10c4R = crate::BitReader;
#[doc = "Field `SCUREGSEC10C4` writer - SCU_REG_SEC1_0C4"]
pub type Scuregsec10c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC10C8` reader - SCU_REG_SEC1_0C8"]
pub type Scuregsec10c8R = crate::BitReader;
#[doc = "Field `SCUREGSEC10C8` writer - SCU_REG_SEC1_0C8"]
pub type Scuregsec10c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC10CC` reader - SCU_REG_SEC1_0CC"]
pub type Scuregsec10ccR = crate::BitReader;
#[doc = "Field `SCUREGSEC10CC` writer - SCU_REG_SEC1_0CC"]
pub type Scuregsec10ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC10D0` reader - SCU_REG_SEC1_0D0"]
pub type Scuregsec10d0R = crate::BitReader;
#[doc = "Field `SCUREGSEC10D0` writer - SCU_REG_SEC1_0D0"]
pub type Scuregsec10d0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC10D4` reader - SCU_REG_SEC1_0D4"]
pub type Scuregsec10d4R = crate::BitReader;
#[doc = "Field `SCUREGSEC10D4` writer - SCU_REG_SEC1_0D4"]
pub type Scuregsec10d4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC10E0` reader - SCU_REG_SEC1_0E0"]
pub type Scuregsec10e0R = crate::BitReader;
#[doc = "Field `SCUREGSEC10E0` writer - SCU_REG_SEC1_0E0"]
pub type Scuregsec10e0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC10F0` reader - SCU_REG_SEC1_0F0"]
pub type Scuregsec10f0R = crate::BitReader;
#[doc = "Field `SCUREGSEC10F0` writer - SCU_REG_SEC1_0F0"]
pub type Scuregsec10f0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - SCU_REG_SEC1_0AC"]
    #[inline(always)]
    pub fn scuregsec10ac(&self) -> Scuregsec10acR {
        Scuregsec10acR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCU_REG_SEC1_0B0"]
    #[inline(always)]
    pub fn scuregsec10b0(&self) -> Scuregsec10b0R {
        Scuregsec10b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_REG_SEC1_0BC"]
    #[inline(always)]
    pub fn scuregsec10bc(&self) -> Scuregsec10bcR {
        Scuregsec10bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_REG_SEC1_0C0"]
    #[inline(always)]
    pub fn scuregsec10c0(&self) -> Scuregsec10c0R {
        Scuregsec10c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC1_0C4"]
    #[inline(always)]
    pub fn scuregsec10c4(&self) -> Scuregsec10c4R {
        Scuregsec10c4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_SEC1_0C8"]
    #[inline(always)]
    pub fn scuregsec10c8(&self) -> Scuregsec10c8R {
        Scuregsec10c8R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_SEC1_0CC"]
    #[inline(always)]
    pub fn scuregsec10cc(&self) -> Scuregsec10ccR {
        Scuregsec10ccR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_REG_SEC1_0D0"]
    #[inline(always)]
    pub fn scuregsec10d0(&self) -> Scuregsec10d0R {
        Scuregsec10d0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_SEC1_0D4"]
    #[inline(always)]
    pub fn scuregsec10d4(&self) -> Scuregsec10d4R {
        Scuregsec10d4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_REG_SEC1_0E0"]
    #[inline(always)]
    pub fn scuregsec10e0(&self) -> Scuregsec10e0R {
        Scuregsec10e0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - SCU_REG_SEC1_0F0"]
    #[inline(always)]
    pub fn scuregsec10f0(&self) -> Scuregsec10f0R {
        Scuregsec10f0R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SCU_REG_SEC1_0AC"]
    #[inline(always)]
    pub fn scuregsec10ac(&mut self) -> Scuregsec10acW<Scuc04Spec> {
        Scuregsec10acW::new(self, 11)
    }
    #[doc = "Bit 12 - SCU_REG_SEC1_0B0"]
    #[inline(always)]
    pub fn scuregsec10b0(&mut self) -> Scuregsec10b0W<Scuc04Spec> {
        Scuregsec10b0W::new(self, 12)
    }
    #[doc = "Bit 15 - SCU_REG_SEC1_0BC"]
    #[inline(always)]
    pub fn scuregsec10bc(&mut self) -> Scuregsec10bcW<Scuc04Spec> {
        Scuregsec10bcW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_REG_SEC1_0C0"]
    #[inline(always)]
    pub fn scuregsec10c0(&mut self) -> Scuregsec10c0W<Scuc04Spec> {
        Scuregsec10c0W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC1_0C4"]
    #[inline(always)]
    pub fn scuregsec10c4(&mut self) -> Scuregsec10c4W<Scuc04Spec> {
        Scuregsec10c4W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_SEC1_0C8"]
    #[inline(always)]
    pub fn scuregsec10c8(&mut self) -> Scuregsec10c8W<Scuc04Spec> {
        Scuregsec10c8W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_SEC1_0CC"]
    #[inline(always)]
    pub fn scuregsec10cc(&mut self) -> Scuregsec10ccW<Scuc04Spec> {
        Scuregsec10ccW::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_REG_SEC1_0D0"]
    #[inline(always)]
    pub fn scuregsec10d0(&mut self) -> Scuregsec10d0W<Scuc04Spec> {
        Scuregsec10d0W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_SEC1_0D4"]
    #[inline(always)]
    pub fn scuregsec10d4(&mut self) -> Scuregsec10d4W<Scuc04Spec> {
        Scuregsec10d4W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_REG_SEC1_0E0"]
    #[inline(always)]
    pub fn scuregsec10e0(&mut self) -> Scuregsec10e0W<Scuc04Spec> {
        Scuregsec10e0W::new(self, 22)
    }
    #[doc = "Bit 28 - SCU_REG_SEC1_0F0"]
    #[inline(always)]
    pub fn scuregsec10f0(&mut self) -> Scuregsec10f0W<Scuc04Spec> {
        Scuregsec10f0W::new(self, 28)
    }
}
#[doc = "Secure1 Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc04Spec;
impl crate::RegisterSpec for Scuc04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc04::R`](R) reader structure"]
impl crate::Readable for Scuc04Spec {}
#[doc = "`write(|w| ..)` method takes [`scuc04::W`](W) writer structure"]
impl crate::Writable for Scuc04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC04 to value 0"]
impl crate::Resettable for Scuc04Spec {}
