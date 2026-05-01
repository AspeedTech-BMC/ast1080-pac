#[doc = "Register `GPIO82C` reader"]
pub type R = crate::R<Gpio82cSpec>;
#[doc = "Register `GPIO82C` writer"]
pub type W = crate::W<Gpio82cSpec>;
#[doc = "Field `GPIO028WrPrivilegeOfMaster` reader - GPIO028 Write Privilege of Master"]
pub type Gpio028wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO028WrPrivilegeOfMaster` writer - GPIO028 Write Privilege of Master"]
pub type Gpio028wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO029WrPrivilegeOfMaster` reader - GPIO029 Write Privilege of Master"]
pub type Gpio029wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO029WrPrivilegeOfMaster` writer - GPIO029 Write Privilege of Master"]
pub type Gpio029wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO030WrPrivilegeOfMaster` reader - GPIO030 Write Privilege of Master"]
pub type Gpio030wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO030WrPrivilegeOfMaster` writer - GPIO030 Write Privilege of Master"]
pub type Gpio030wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO031WrPrivilegeOfMaster` reader - GPIO031 Write Privilege of Master"]
pub type Gpio031wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO031WrPrivilegeOfMaster` writer - GPIO031 Write Privilege of Master"]
pub type Gpio031wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO028 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio028wr_privilege_of_master(&self) -> Gpio028wrPrivilegeOfMasterR {
        Gpio028wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO029 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio029wr_privilege_of_master(&self) -> Gpio029wrPrivilegeOfMasterR {
        Gpio029wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO030 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio030wr_privilege_of_master(&self) -> Gpio030wrPrivilegeOfMasterR {
        Gpio030wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO031 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio031wr_privilege_of_master(&self) -> Gpio031wrPrivilegeOfMasterR {
        Gpio031wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO028 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio028wr_privilege_of_master(&mut self) -> Gpio028wrPrivilegeOfMasterW<Gpio82cSpec> {
        Gpio028wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO029 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio029wr_privilege_of_master(&mut self) -> Gpio029wrPrivilegeOfMasterW<Gpio82cSpec> {
        Gpio029wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO030 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio030wr_privilege_of_master(&mut self) -> Gpio030wrPrivilegeOfMasterW<Gpio82cSpec> {
        Gpio030wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO031 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio031wr_privilege_of_master(&mut self) -> Gpio031wrPrivilegeOfMasterW<Gpio82cSpec> {
        Gpio031wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio82c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio82c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio82cSpec;
impl crate::RegisterSpec for Gpio82cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio82c::R`](R) reader structure"]
impl crate::Readable for Gpio82cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio82c::W`](W) writer structure"]
impl crate::Writable for Gpio82cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO82C to value 0xffff_ffff"]
impl crate::Resettable for Gpio82cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
