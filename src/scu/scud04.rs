#[doc = "Register `SCUD04` reader"]
pub type R = crate::R<Scud04Spec>;
#[doc = "Register `SCUD04` writer"]
pub type W = crate::W<Scud04Spec>;
#[doc = "Field `SCUREGSEC39AC` reader - SCU_REG_SEC3_9AC"]
pub type Scuregsec39acR = crate::BitReader;
#[doc = "Field `SCUREGSEC39AC` writer - SCU_REG_SEC3_9AC"]
pub type Scuregsec39acW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC30B0` reader - SCU_REG_SEC3_0B0"]
pub type Scuregsec30b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC30B0` writer - SCU_REG_SEC3_0B0"]
pub type Scuregsec30b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGSEC30BC` reader - SCU_REG_SEC3_0BC"]
pub type Scuregsec30bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC30BC` writer - SCU_REG_SEC3_0BC"]
pub type Scuregsec30bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC30C0` reader - SCU_REG_SEC3_0C0"]
pub type Scuregsec30c0R = crate::BitReader;
#[doc = "Field `SCUREGSEC30C0` writer - SCU_REG_SEC3_0C0"]
pub type Scuregsec30c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC30C4` reader - SCU_REG_SEC3_0C4"]
pub type Scuregsec30c4R = crate::BitReader;
#[doc = "Field `SCUREGSEC30C4` writer - SCU_REG_SEC3_0C4"]
pub type Scuregsec30c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC30C8` reader - SCU_REG_SEC3_0C8"]
pub type Scuregsec30c8R = crate::BitReader;
#[doc = "Field `SCUREGSEC30C8` writer - SCU_REG_SEC3_0C8"]
pub type Scuregsec30c8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC30CC` reader - SCU_REG_SEC3_0CC"]
pub type Scuregsec30ccR = crate::BitReader;
#[doc = "Field `SCUREGSEC30CC` writer - SCU_REG_SEC3_0CC"]
pub type Scuregsec30ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC30D0` reader - SCU_REG_SEC3_0D0"]
pub type Scuregsec30d0R = crate::BitReader;
#[doc = "Field `SCUREGSEC30D0` writer - SCU_REG_SEC3_0D0"]
pub type Scuregsec30d0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC30D4` reader - SCU_REG_SEC3_0D4"]
pub type Scuregsec30d4R = crate::BitReader;
#[doc = "Field `SCUREGSEC30D4` writer - SCU_REG_SEC3_0D4"]
pub type Scuregsec30d4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC30E0` reader - SCU_REG_SEC3_0E0"]
pub type Scuregsec30e0R = crate::BitReader;
#[doc = "Field `SCUREGSEC30E0` writer - SCU_REG_SEC3_0E0"]
pub type Scuregsec30e0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC30F0` reader - SCU_REG_SEC3_0F0"]
pub type Scuregsec30f0R = crate::BitReader;
#[doc = "Field `SCUREGSEC30F0` writer - SCU_REG_SEC3_0F0"]
pub type Scuregsec30f0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 11 - SCU_REG_SEC3_9AC"]
    #[inline(always)]
    pub fn scuregsec39ac(&self) -> Scuregsec39acR {
        Scuregsec39acR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCU_REG_SEC3_0B0"]
    #[inline(always)]
    pub fn scuregsec30b0(&self) -> Scuregsec30b0R {
        Scuregsec30b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SCU_REG_SEC3_0BC"]
    #[inline(always)]
    pub fn scuregsec30bc(&self) -> Scuregsec30bcR {
        Scuregsec30bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_REG_SEC3_0C0"]
    #[inline(always)]
    pub fn scuregsec30c0(&self) -> Scuregsec30c0R {
        Scuregsec30c0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_REG_SEC3_0C4"]
    #[inline(always)]
    pub fn scuregsec30c4(&self) -> Scuregsec30c4R {
        Scuregsec30c4R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_REG_SEC3_0C8"]
    #[inline(always)]
    pub fn scuregsec30c8(&self) -> Scuregsec30c8R {
        Scuregsec30c8R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCU_REG_SEC3_0CC"]
    #[inline(always)]
    pub fn scuregsec30cc(&self) -> Scuregsec30ccR {
        Scuregsec30ccR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SCU_REG_SEC3_0D0"]
    #[inline(always)]
    pub fn scuregsec30d0(&self) -> Scuregsec30d0R {
        Scuregsec30d0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SCU_REG_SEC3_0D4"]
    #[inline(always)]
    pub fn scuregsec30d4(&self) -> Scuregsec30d4R {
        Scuregsec30d4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SCU_REG_SEC3_0E0"]
    #[inline(always)]
    pub fn scuregsec30e0(&self) -> Scuregsec30e0R {
        Scuregsec30e0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - SCU_REG_SEC3_0F0"]
    #[inline(always)]
    pub fn scuregsec30f0(&self) -> Scuregsec30f0R {
        Scuregsec30f0R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - SCU_REG_SEC3_9AC"]
    #[inline(always)]
    pub fn scuregsec39ac(&mut self) -> Scuregsec39acW<Scud04Spec> {
        Scuregsec39acW::new(self, 11)
    }
    #[doc = "Bit 12 - SCU_REG_SEC3_0B0"]
    #[inline(always)]
    pub fn scuregsec30b0(&mut self) -> Scuregsec30b0W<Scud04Spec> {
        Scuregsec30b0W::new(self, 12)
    }
    #[doc = "Bit 15 - SCU_REG_SEC3_0BC"]
    #[inline(always)]
    pub fn scuregsec30bc(&mut self) -> Scuregsec30bcW<Scud04Spec> {
        Scuregsec30bcW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_REG_SEC3_0C0"]
    #[inline(always)]
    pub fn scuregsec30c0(&mut self) -> Scuregsec30c0W<Scud04Spec> {
        Scuregsec30c0W::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_REG_SEC3_0C4"]
    #[inline(always)]
    pub fn scuregsec30c4(&mut self) -> Scuregsec30c4W<Scud04Spec> {
        Scuregsec30c4W::new(self, 17)
    }
    #[doc = "Bit 18 - SCU_REG_SEC3_0C8"]
    #[inline(always)]
    pub fn scuregsec30c8(&mut self) -> Scuregsec30c8W<Scud04Spec> {
        Scuregsec30c8W::new(self, 18)
    }
    #[doc = "Bit 19 - SCU_REG_SEC3_0CC"]
    #[inline(always)]
    pub fn scuregsec30cc(&mut self) -> Scuregsec30ccW<Scud04Spec> {
        Scuregsec30ccW::new(self, 19)
    }
    #[doc = "Bit 20 - SCU_REG_SEC3_0D0"]
    #[inline(always)]
    pub fn scuregsec30d0(&mut self) -> Scuregsec30d0W<Scud04Spec> {
        Scuregsec30d0W::new(self, 20)
    }
    #[doc = "Bit 21 - SCU_REG_SEC3_0D4"]
    #[inline(always)]
    pub fn scuregsec30d4(&mut self) -> Scuregsec30d4W<Scud04Spec> {
        Scuregsec30d4W::new(self, 21)
    }
    #[doc = "Bit 22 - SCU_REG_SEC3_0E0"]
    #[inline(always)]
    pub fn scuregsec30e0(&mut self) -> Scuregsec30e0W<Scud04Spec> {
        Scuregsec30e0W::new(self, 22)
    }
    #[doc = "Bit 28 - SCU_REG_SEC3_0F0"]
    #[inline(always)]
    pub fn scuregsec30f0(&mut self) -> Scuregsec30f0W<Scud04Spec> {
        Scuregsec30f0W::new(self, 28)
    }
}
#[doc = "Secure3 Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud04Spec;
impl crate::RegisterSpec for Scud04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud04::R`](R) reader structure"]
impl crate::Readable for Scud04Spec {}
#[doc = "`write(|w| ..)` method takes [`scud04::W`](W) writer structure"]
impl crate::Writable for Scud04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD04 to value 0"]
impl crate::Resettable for Scud04Spec {}
