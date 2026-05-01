#[doc = "Register `I2C44` reader"]
pub type R = crate::R<I2c44Spec>;
#[doc = "Register `I2C44` writer"]
pub type W = crate::W<I2c44Spec>;
#[doc = "Field `SADDR4` reader - SADDR4"]
pub type Saddr4R = crate::FieldReader;
#[doc = "Field `SADDR4` writer - SADDR4"]
pub type Saddr4W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADDR4EN` reader - SADDR4_EN"]
pub type Saddr4enR = crate::BitReader;
#[doc = "Field `SADDR4EN` writer - SADDR4_EN"]
pub type Saddr4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR5` reader - SADDR5"]
pub type Saddr5R = crate::FieldReader;
#[doc = "Field `SADDR5` writer - SADDR5"]
pub type Saddr5W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADDR5EN` reader - SADDR5_EN"]
pub type Saddr5enR = crate::BitReader;
#[doc = "Field `SADDR5EN` writer - SADDR5_EN"]
pub type Saddr5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR6` reader - SADDR6"]
pub type Saddr6R = crate::FieldReader;
#[doc = "Field `SADDR6` writer - SADDR6"]
pub type Saddr6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADDR6EN` reader - SADDR6_EN"]
pub type Saddr6enR = crate::BitReader;
#[doc = "Field `SADDR6EN` writer - SADDR6_EN"]
pub type Saddr6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR7` reader - SADDR7"]
pub type Saddr7R = crate::FieldReader;
#[doc = "Field `SADDR7` writer - SADDR7"]
pub type Saddr7W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADDR7EN` reader - SADDR7_EN"]
pub type Saddr7enR = crate::BitReader;
#[doc = "Field `SADDR7EN` writer - SADDR7_EN"]
pub type Saddr7enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - SADDR4"]
    #[inline(always)]
    pub fn saddr4(&self) -> Saddr4R {
        Saddr4R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - SADDR4_EN"]
    #[inline(always)]
    pub fn saddr4en(&self) -> Saddr4enR {
        Saddr4enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - SADDR5"]
    #[inline(always)]
    pub fn saddr5(&self) -> Saddr5R {
        Saddr5R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - SADDR5_EN"]
    #[inline(always)]
    pub fn saddr5en(&self) -> Saddr5enR {
        Saddr5enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - SADDR6"]
    #[inline(always)]
    pub fn saddr6(&self) -> Saddr6R {
        Saddr6R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - SADDR6_EN"]
    #[inline(always)]
    pub fn saddr6en(&self) -> Saddr6enR {
        Saddr6enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - SADDR7"]
    #[inline(always)]
    pub fn saddr7(&self) -> Saddr7R {
        Saddr7R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - SADDR7_EN"]
    #[inline(always)]
    pub fn saddr7en(&self) -> Saddr7enR {
        Saddr7enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - SADDR4"]
    #[inline(always)]
    pub fn saddr4(&mut self) -> Saddr4W<I2c44Spec> {
        Saddr4W::new(self, 0)
    }
    #[doc = "Bit 7 - SADDR4_EN"]
    #[inline(always)]
    pub fn saddr4en(&mut self) -> Saddr4enW<I2c44Spec> {
        Saddr4enW::new(self, 7)
    }
    #[doc = "Bits 8:14 - SADDR5"]
    #[inline(always)]
    pub fn saddr5(&mut self) -> Saddr5W<I2c44Spec> {
        Saddr5W::new(self, 8)
    }
    #[doc = "Bit 15 - SADDR5_EN"]
    #[inline(always)]
    pub fn saddr5en(&mut self) -> Saddr5enW<I2c44Spec> {
        Saddr5enW::new(self, 15)
    }
    #[doc = "Bits 16:22 - SADDR6"]
    #[inline(always)]
    pub fn saddr6(&mut self) -> Saddr6W<I2c44Spec> {
        Saddr6W::new(self, 16)
    }
    #[doc = "Bit 23 - SADDR6_EN"]
    #[inline(always)]
    pub fn saddr6en(&mut self) -> Saddr6enW<I2c44Spec> {
        Saddr6enW::new(self, 23)
    }
    #[doc = "Bits 24:30 - SADDR7"]
    #[inline(always)]
    pub fn saddr7(&mut self) -> Saddr7W<I2c44Spec> {
        Saddr7W::new(self, 24)
    }
    #[doc = "Bit 31 - SADDR7_EN"]
    #[inline(always)]
    pub fn saddr7en(&mut self) -> Saddr7enW<I2c44Spec> {
        Saddr7enW::new(self, 31)
    }
}
#[doc = "Slave Device Address Register (extra)\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c44Spec;
impl crate::RegisterSpec for I2c44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c44::R`](R) reader structure"]
impl crate::Readable for I2c44Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c44::W`](W) writer structure"]
impl crate::Writable for I2c44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C44 to value 0"]
impl crate::Resettable for I2c44Spec {}
