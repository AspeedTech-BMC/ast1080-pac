#[doc = "Register `SCUF14` reader"]
pub type R = crate::R<Scuf14Spec>;
#[doc = "Register `SCUF14` writer"]
pub type W = crate::W<Scuf14Spec>;
#[doc = "Field `SCUREGRST280` reader - SCU_REG_RST_280"]
pub type Scuregrst280R = crate::BitReader;
#[doc = "Field `SCUREGRST280` writer - SCU_REG_RST_280"]
pub type Scuregrst280W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST284` reader - SCU_REG_RST_284"]
pub type Scuregrst284R = crate::BitReader;
#[doc = "Field `SCUREGRST284` writer - SCU_REG_RST_284"]
pub type Scuregrst284W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUREGRST2A0` reader - SCU_REG_RST_2A0"]
pub type Scuregrst2a0R = crate::BitReader;
#[doc = "Field `SCUREGRST2A0` writer - SCU_REG_RST_2A0"]
pub type Scuregrst2a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2A4` reader - SCU_REG_RST_2A4"]
pub type Scuregrst2a4R = crate::BitReader;
#[doc = "Field `SCUREGRST2A4` writer - SCU_REG_RST_2A4"]
pub type Scuregrst2a4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2A8` reader - SCU_REG_RST_2A8"]
pub type Scuregrst2a8R = crate::BitReader;
#[doc = "Field `SCUREGRST2A8` writer - SCU_REG_RST_2A8"]
pub type Scuregrst2a8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2AC` reader - SCU_REG_RST_2AC"]
pub type Scuregrst2acR = crate::BitReader;
#[doc = "Field `SCUREGRST2AC` writer - SCU_REG_RST_2AC"]
pub type Scuregrst2acW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2B0` reader - SCU_REG_RST_2B0"]
pub type Scuregrst2b0R = crate::BitReader;
#[doc = "Field `SCUREGRST2B0` writer - SCU_REG_RST_2B0"]
pub type Scuregrst2b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2B4` reader - SCU_REG_RST_2B4"]
pub type Scuregrst2b4R = crate::BitReader;
#[doc = "Field `SCUREGRST2B4` writer - SCU_REG_RST_2B4"]
pub type Scuregrst2b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2B8` reader - SCU_REG_RST_2B8"]
pub type Scuregrst2b8R = crate::BitReader;
#[doc = "Field `SCUREGRST2B8` writer - SCU_REG_RST_2B8"]
pub type Scuregrst2b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2BC` reader - SCU_REG_RST_2BC"]
pub type Scuregrst2bcR = crate::BitReader;
#[doc = "Field `SCUREGRST2BC` writer - SCU_REG_RST_2BC"]
pub type Scuregrst2bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `SCUREGRST2F0` reader - SCU_REG_RST_2F0"]
pub type Scuregrst2f0R = crate::BitReader;
#[doc = "Field `SCUREGRST2F0` writer - SCU_REG_RST_2F0"]
pub type Scuregrst2f0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2F4` reader - SCU_REG_RST_2F4"]
pub type Scuregrst2f4R = crate::BitReader;
#[doc = "Field `SCUREGRST2F4` writer - SCU_REG_RST_2F4"]
pub type Scuregrst2f4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGRST2F8` reader - SCU_REG_RST_2F8"]
pub type Scuregrst2f8R = crate::BitReader;
#[doc = "Field `SCUREGRST2F8` writer - SCU_REG_RST_2F8"]
pub type Scuregrst2f8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_RST_280"]
    #[inline(always)]
    pub fn scuregrst280(&self) -> Scuregrst280R {
        Scuregrst280R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_284"]
    #[inline(always)]
    pub fn scuregrst284(&self) -> Scuregrst284R {
        Scuregrst284R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - SCU_REG_RST_2A0"]
    #[inline(always)]
    pub fn scuregrst2a0(&self) -> Scuregrst2a0R {
        Scuregrst2a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_RST_2A4"]
    #[inline(always)]
    pub fn scuregrst2a4(&self) -> Scuregrst2a4R {
        Scuregrst2a4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SCU_REG_RST_2A8"]
    #[inline(always)]
    pub fn scuregrst2a8(&self) -> Scuregrst2a8R {
        Scuregrst2a8R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCU_REG_RST_2AC"]
    #[inline(always)]
    pub fn scuregrst2ac(&self) -> Scuregrst2acR {
        Scuregrst2acR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCU_REG_RST_2B0"]
    #[inline(always)]
    pub fn scuregrst2b0(&self) -> Scuregrst2b0R {
        Scuregrst2b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_RST_2B4"]
    #[inline(always)]
    pub fn scuregrst2b4(&self) -> Scuregrst2b4R {
        Scuregrst2b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_RST_2B8"]
    #[inline(always)]
    pub fn scuregrst2b8(&self) -> Scuregrst2b8R {
        Scuregrst2b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_RST_2BC"]
    #[inline(always)]
    pub fn scuregrst2bc(&self) -> Scuregrst2bcR {
        Scuregrst2bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 28 - SCU_REG_RST_2F0"]
    #[inline(always)]
    pub fn scuregrst2f0(&self) -> Scuregrst2f0R {
        Scuregrst2f0R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SCU_REG_RST_2F4"]
    #[inline(always)]
    pub fn scuregrst2f4(&self) -> Scuregrst2f4R {
        Scuregrst2f4R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SCU_REG_RST_2F8"]
    #[inline(always)]
    pub fn scuregrst2f8(&self) -> Scuregrst2f8R {
        Scuregrst2f8R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_RST_280"]
    #[inline(always)]
    pub fn scuregrst280(&mut self) -> Scuregrst280W<Scuf14Spec> {
        Scuregrst280W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_RST_284"]
    #[inline(always)]
    pub fn scuregrst284(&mut self) -> Scuregrst284W<Scuf14Spec> {
        Scuregrst284W::new(self, 1)
    }
    #[doc = "Bit 8 - SCU_REG_RST_2A0"]
    #[inline(always)]
    pub fn scuregrst2a0(&mut self) -> Scuregrst2a0W<Scuf14Spec> {
        Scuregrst2a0W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_RST_2A4"]
    #[inline(always)]
    pub fn scuregrst2a4(&mut self) -> Scuregrst2a4W<Scuf14Spec> {
        Scuregrst2a4W::new(self, 9)
    }
    #[doc = "Bit 10 - SCU_REG_RST_2A8"]
    #[inline(always)]
    pub fn scuregrst2a8(&mut self) -> Scuregrst2a8W<Scuf14Spec> {
        Scuregrst2a8W::new(self, 10)
    }
    #[doc = "Bit 11 - SCU_REG_RST_2AC"]
    #[inline(always)]
    pub fn scuregrst2ac(&mut self) -> Scuregrst2acW<Scuf14Spec> {
        Scuregrst2acW::new(self, 11)
    }
    #[doc = "Bit 12 - SCU_REG_RST_2B0"]
    #[inline(always)]
    pub fn scuregrst2b0(&mut self) -> Scuregrst2b0W<Scuf14Spec> {
        Scuregrst2b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_RST_2B4"]
    #[inline(always)]
    pub fn scuregrst2b4(&mut self) -> Scuregrst2b4W<Scuf14Spec> {
        Scuregrst2b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_RST_2B8"]
    #[inline(always)]
    pub fn scuregrst2b8(&mut self) -> Scuregrst2b8W<Scuf14Spec> {
        Scuregrst2b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_RST_2BC"]
    #[inline(always)]
    pub fn scuregrst2bc(&mut self) -> Scuregrst2bcW<Scuf14Spec> {
        Scuregrst2bcW::new(self, 15)
    }
    #[doc = "Bit 28 - SCU_REG_RST_2F0"]
    #[inline(always)]
    pub fn scuregrst2f0(&mut self) -> Scuregrst2f0W<Scuf14Spec> {
        Scuregrst2f0W::new(self, 28)
    }
    #[doc = "Bit 29 - SCU_REG_RST_2F4"]
    #[inline(always)]
    pub fn scuregrst2f4(&mut self) -> Scuregrst2f4W<Scuf14Spec> {
        Scuregrst2f4W::new(self, 29)
    }
    #[doc = "Bit 30 - SCU_REG_RST_2F8"]
    #[inline(always)]
    pub fn scuregrst2f8(&mut self) -> Scuregrst2f8W<Scuf14Spec> {
        Scuregrst2f8W::new(self, 30)
    }
}
#[doc = "Reset Control 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf14Spec;
impl crate::RegisterSpec for Scuf14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf14::R`](R) reader structure"]
impl crate::Readable for Scuf14Spec {}
#[doc = "`write(|w| ..)` method takes [`scuf14::W`](W) writer structure"]
impl crate::Writable for Scuf14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF14 to value 0"]
impl crate::Resettable for Scuf14Spec {}
