#[doc = "Register `GPIO898` reader"]
pub type R = crate::R<Gpio898Spec>;
#[doc = "Register `GPIO898` writer"]
pub type W = crate::W<Gpio898Spec>;
#[doc = "Field `GPIO136WrPrivilegeOfMaster` reader - GPIO136 Write Privilege of Master"]
pub type Gpio136wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO136WrPrivilegeOfMaster` writer - GPIO136 Write Privilege of Master"]
pub type Gpio136wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO137WrPrivilegeOfMaster` reader - GPIO137 Write Privilege of Master"]
pub type Gpio137wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO137WrPrivilegeOfMaster` writer - GPIO137 Write Privilege of Master"]
pub type Gpio137wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO138WrPrivilegeOfMaster` reader - GPIO138 Write Privilege of Master"]
pub type Gpio138wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO138WrPrivilegeOfMaster` writer - GPIO138 Write Privilege of Master"]
pub type Gpio138wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO139WrPrivilegeOfMaster` reader - GPIO139 Write Privilege of Master"]
pub type Gpio139wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO139WrPrivilegeOfMaster` writer - GPIO139 Write Privilege of Master"]
pub type Gpio139wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO136 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio136wr_privilege_of_master(&self) -> Gpio136wrPrivilegeOfMasterR {
        Gpio136wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO137 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio137wr_privilege_of_master(&self) -> Gpio137wrPrivilegeOfMasterR {
        Gpio137wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO138 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio138wr_privilege_of_master(&self) -> Gpio138wrPrivilegeOfMasterR {
        Gpio138wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO139 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio139wr_privilege_of_master(&self) -> Gpio139wrPrivilegeOfMasterR {
        Gpio139wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO136 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio136wr_privilege_of_master(&mut self) -> Gpio136wrPrivilegeOfMasterW<Gpio898Spec> {
        Gpio136wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO137 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio137wr_privilege_of_master(&mut self) -> Gpio137wrPrivilegeOfMasterW<Gpio898Spec> {
        Gpio137wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO138 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio138wr_privilege_of_master(&mut self) -> Gpio138wrPrivilegeOfMasterW<Gpio898Spec> {
        Gpio138wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO139 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio139wr_privilege_of_master(&mut self) -> Gpio139wrPrivilegeOfMasterW<Gpio898Spec> {
        Gpio139wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#34\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio898::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio898::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio898Spec;
impl crate::RegisterSpec for Gpio898Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio898::R`](R) reader structure"]
impl crate::Readable for Gpio898Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio898::W`](W) writer structure"]
impl crate::Writable for Gpio898Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO898 to value 0xffff_ffff"]
impl crate::Resettable for Gpio898Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
