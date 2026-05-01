#[doc = "Register `I2C40` reader"]
pub type R = crate::R<I2c40Spec>;
#[doc = "Register `I2C40` writer"]
pub type W = crate::W<I2c40Spec>;
#[doc = "Field `SADDR0` reader - SADDR0"]
pub type Saddr0R = crate::FieldReader;
#[doc = "Field `SADDR0` writer - SADDR0"]
pub type Saddr0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADDR0EN` reader - SADDR0_EN"]
pub type Saddr0enR = crate::BitReader;
#[doc = "Field `SADDR0EN` writer - SADDR0_EN"]
pub type Saddr0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR1` reader - SADDR1"]
pub type Saddr1R = crate::FieldReader;
#[doc = "Field `SADDR1` writer - SADDR1"]
pub type Saddr1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADDR1EN` reader - SADDR1_EN"]
pub type Saddr1enR = crate::BitReader;
#[doc = "Field `SADDR1EN` writer - SADDR1_EN"]
pub type Saddr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR2` reader - SADDR2"]
pub type Saddr2R = crate::FieldReader;
#[doc = "Field `SADDR2` writer - SADDR2"]
pub type Saddr2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADDR2EN` reader - SADDR2_EN"]
pub type Saddr2enR = crate::BitReader;
#[doc = "Field `SADDR2EN` writer - SADDR2_EN"]
pub type Saddr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SADDR3` reader - SADDR3"]
pub type Saddr3R = crate::FieldReader;
#[doc = "Field `SADDR3` writer - SADDR3"]
pub type Saddr3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SADDR3EN` reader - SADDR3_EN"]
pub type Saddr3enR = crate::BitReader;
#[doc = "Field `SADDR3EN` writer - SADDR3_EN"]
pub type Saddr3enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - SADDR0"]
    #[inline(always)]
    pub fn saddr0(&self) -> Saddr0R {
        Saddr0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - SADDR0_EN"]
    #[inline(always)]
    pub fn saddr0en(&self) -> Saddr0enR {
        Saddr0enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - SADDR1"]
    #[inline(always)]
    pub fn saddr1(&self) -> Saddr1R {
        Saddr1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - SADDR1_EN"]
    #[inline(always)]
    pub fn saddr1en(&self) -> Saddr1enR {
        Saddr1enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - SADDR2"]
    #[inline(always)]
    pub fn saddr2(&self) -> Saddr2R {
        Saddr2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - SADDR2_EN"]
    #[inline(always)]
    pub fn saddr2en(&self) -> Saddr2enR {
        Saddr2enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - SADDR3"]
    #[inline(always)]
    pub fn saddr3(&self) -> Saddr3R {
        Saddr3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - SADDR3_EN"]
    #[inline(always)]
    pub fn saddr3en(&self) -> Saddr3enR {
        Saddr3enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - SADDR0"]
    #[inline(always)]
    pub fn saddr0(&mut self) -> Saddr0W<I2c40Spec> {
        Saddr0W::new(self, 0)
    }
    #[doc = "Bit 7 - SADDR0_EN"]
    #[inline(always)]
    pub fn saddr0en(&mut self) -> Saddr0enW<I2c40Spec> {
        Saddr0enW::new(self, 7)
    }
    #[doc = "Bits 8:14 - SADDR1"]
    #[inline(always)]
    pub fn saddr1(&mut self) -> Saddr1W<I2c40Spec> {
        Saddr1W::new(self, 8)
    }
    #[doc = "Bit 15 - SADDR1_EN"]
    #[inline(always)]
    pub fn saddr1en(&mut self) -> Saddr1enW<I2c40Spec> {
        Saddr1enW::new(self, 15)
    }
    #[doc = "Bits 16:22 - SADDR2"]
    #[inline(always)]
    pub fn saddr2(&mut self) -> Saddr2W<I2c40Spec> {
        Saddr2W::new(self, 16)
    }
    #[doc = "Bit 23 - SADDR2_EN"]
    #[inline(always)]
    pub fn saddr2en(&mut self) -> Saddr2enW<I2c40Spec> {
        Saddr2enW::new(self, 23)
    }
    #[doc = "Bits 24:30 - SADDR3"]
    #[inline(always)]
    pub fn saddr3(&mut self) -> Saddr3W<I2c40Spec> {
        Saddr3W::new(self, 24)
    }
    #[doc = "Bit 31 - SADDR3_EN"]
    #[inline(always)]
    pub fn saddr3en(&mut self) -> Saddr3enW<I2c40Spec> {
        Saddr3enW::new(self, 31)
    }
}
#[doc = "Slave Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c40Spec;
impl crate::RegisterSpec for I2c40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c40::R`](R) reader structure"]
impl crate::Readable for I2c40Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c40::W`](W) writer structure"]
impl crate::Writable for I2c40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C40 to value 0"]
impl crate::Resettable for I2c40Spec {}
