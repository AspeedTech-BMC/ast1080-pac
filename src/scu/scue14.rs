#[doc = "Register `SCUE14` reader"]
pub type R = crate::R<Scue14Spec>;
#[doc = "Register `SCUE14` writer"]
pub type W = crate::W<Scue14Spec>;
#[doc = "Field `SCUREGLOCK2A0` reader - SCU_REG_LOCK_2A0"]
pub type Scureglock2a0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2A0` writer - SCU_REG_LOCK_2A0"]
pub type Scureglock2a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2A4` reader - SCU_REG_LOCK_2A4"]
pub type Scureglock2a4R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2A4` writer - SCU_REG_LOCK_2A4"]
pub type Scureglock2a4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2A8` reader - SCU_REG_LOCK_2A8"]
pub type Scureglock2a8R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2A8` writer - SCU_REG_LOCK_2A8"]
pub type Scureglock2a8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2AC` reader - SCU_REG_LOCK_2AC"]
pub type Scureglock2acR = crate::BitReader;
#[doc = "Field `SCUREGLOCK2AC` writer - SCU_REG_LOCK_2AC"]
pub type Scureglock2acW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2B0` reader - SCU_REG_LOCK_2B0"]
pub type Scureglock2b0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2B0` writer - SCU_REG_LOCK_2B0"]
pub type Scureglock2b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2B4` reader - SCU_REG_LOCK_2B4"]
pub type Scureglock2b4R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2B4` writer - SCU_REG_LOCK_2B4"]
pub type Scureglock2b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2B8` reader - SCU_REG_LOCK_2B8"]
pub type Scureglock2b8R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2B8` writer - SCU_REG_LOCK_2B8"]
pub type Scureglock2b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2BC` reader - SCU_REG_LOCK_2BC"]
pub type Scureglock2bcR = crate::BitReader;
#[doc = "Field `SCUREGLOCK2BC` writer - SCU_REG_LOCK_2BC"]
pub type Scureglock2bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `SCUREGLOCK2F0` reader - SCU_REG_LOCK_2F0"]
pub type Scureglock2f0R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2F0` writer - SCU_REG_LOCK_2F0"]
pub type Scureglock2f0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2F4` reader - SCU_REG_LOCK_2F4"]
pub type Scureglock2f4R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2F4` writer - SCU_REG_LOCK_2F4"]
pub type Scureglock2f4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGLOCK2F8` reader - SCU_REG_LOCK_2F8"]
pub type Scureglock2f8R = crate::BitReader;
#[doc = "Field `SCUREGLOCK2F8` writer - SCU_REG_LOCK_2F8"]
pub type Scureglock2f8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - SCU_REG_LOCK_2A0"]
    #[inline(always)]
    pub fn scureglock2a0(&self) -> Scureglock2a0R {
        Scureglock2a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_LOCK_2A4"]
    #[inline(always)]
    pub fn scureglock2a4(&self) -> Scureglock2a4R {
        Scureglock2a4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SCU_REG_LOCK_2A8"]
    #[inline(always)]
    pub fn scureglock2a8(&self) -> Scureglock2a8R {
        Scureglock2a8R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCU_REG_LOCK_2AC"]
    #[inline(always)]
    pub fn scureglock2ac(&self) -> Scureglock2acR {
        Scureglock2acR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_2B0"]
    #[inline(always)]
    pub fn scureglock2b0(&self) -> Scureglock2b0R {
        Scureglock2b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_2B4"]
    #[inline(always)]
    pub fn scureglock2b4(&self) -> Scureglock2b4R {
        Scureglock2b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_2B8"]
    #[inline(always)]
    pub fn scureglock2b8(&self) -> Scureglock2b8R {
        Scureglock2b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_2BC"]
    #[inline(always)]
    pub fn scureglock2bc(&self) -> Scureglock2bcR {
        Scureglock2bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - SCU_REG_LOCK_2F0"]
    #[inline(always)]
    pub fn scureglock2f0(&self) -> Scureglock2f0R {
        Scureglock2f0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_REG_LOCK_2F4"]
    #[inline(always)]
    pub fn scureglock2f4(&self) -> Scureglock2f4R {
        Scureglock2f4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCU_REG_LOCK_2F8"]
    #[inline(always)]
    pub fn scureglock2f8(&self) -> Scureglock2f8R {
        Scureglock2f8R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SCU_REG_LOCK_2A0"]
    #[inline(always)]
    pub fn scureglock2a0(&mut self) -> Scureglock2a0W<Scue14Spec> {
        Scureglock2a0W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_LOCK_2A4"]
    #[inline(always)]
    pub fn scureglock2a4(&mut self) -> Scureglock2a4W<Scue14Spec> {
        Scureglock2a4W::new(self, 9)
    }
    #[doc = "Bit 10 - SCU_REG_LOCK_2A8"]
    #[inline(always)]
    pub fn scureglock2a8(&mut self) -> Scureglock2a8W<Scue14Spec> {
        Scureglock2a8W::new(self, 10)
    }
    #[doc = "Bit 11 - SCU_REG_LOCK_2AC"]
    #[inline(always)]
    pub fn scureglock2ac(&mut self) -> Scureglock2acW<Scue14Spec> {
        Scureglock2acW::new(self, 11)
    }
    #[doc = "Bit 12 - SCU_REG_LOCK_2B0"]
    #[inline(always)]
    pub fn scureglock2b0(&mut self) -> Scureglock2b0W<Scue14Spec> {
        Scureglock2b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_LOCK_2B4"]
    #[inline(always)]
    pub fn scureglock2b4(&mut self) -> Scureglock2b4W<Scue14Spec> {
        Scureglock2b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_LOCK_2B8"]
    #[inline(always)]
    pub fn scureglock2b8(&mut self) -> Scureglock2b8W<Scue14Spec> {
        Scureglock2b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_LOCK_2BC"]
    #[inline(always)]
    pub fn scureglock2bc(&mut self) -> Scureglock2bcW<Scue14Spec> {
        Scureglock2bcW::new(self, 15)
    }
    #[doc = "Bit 28 - SCU_REG_LOCK_2F0"]
    #[inline(always)]
    pub fn scureglock2f0(&mut self) -> Scureglock2f0W<Scue14Spec> {
        Scureglock2f0W::new(self, 28)
    }
    #[doc = "Bit 29 - SCU_REG_LOCK_2F4"]
    #[inline(always)]
    pub fn scureglock2f4(&mut self) -> Scureglock2f4W<Scue14Spec> {
        Scureglock2f4W::new(self, 29)
    }
    #[doc = "Bit 30 - SCU_REG_LOCK_2F8"]
    #[inline(always)]
    pub fn scureglock2f8(&mut self) -> Scureglock2f8W<Scue14Spec> {
        Scureglock2f8W::new(self, 30)
    }
}
#[doc = "Write Protection 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue14Spec;
impl crate::RegisterSpec for Scue14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue14::R`](R) reader structure"]
impl crate::Readable for Scue14Spec {}
#[doc = "`write(|w| ..)` method takes [`scue14::W`](W) writer structure"]
impl crate::Writable for Scue14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE14 to value 0"]
impl crate::Resettable for Scue14Spec {}
